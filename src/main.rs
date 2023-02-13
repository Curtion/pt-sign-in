extern crate chrono;
extern crate job_scheduler;
extern crate cloudflare_bypasser;

use chrono::prelude::*;
use clap::Parser;
use job_scheduler::{Job, JobScheduler};
use reqwest::blocking::Client;
use std::{collections::HashMap, time::Duration};

static UA:&'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36 Edg/105.0.1343.50";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    password: String,
}

fn main() {
    let args = Args::parse();

    let mut sched = JobScheduler::new();

    job(&args.user, &args.password); // 开始时运行一次

    println!("每天 09:00 会自动执行登录与签到任务!");

    //0 0 9 * * * *
    sched.add(Job::new("0 0 9 * * * *".parse().unwrap(), || {
        job(&args.user, &args.password);
    }));
    loop {
        sched.tick();
        std::thread::sleep(Duration::from_secs(5));
    }
}

fn job(user: &str, password: &str) {
    let client = Client::builder()
        .user_agent(UA)
        .cookie_store(true)
        .build()
        .expect("构建请求失败");
    login_in(&client, user, password);
    sign_in(&client);
}

fn login_in(client: &Client, user: &str, password: &str) -> () {
    let mut params = HashMap::new();
    params.insert("password", password);
    params.insert("username", user);

    let resp = client
        .post("https://www.hdarea.co/takelogin.php")
        .form(&params)
        .send();
    match resp {
        Ok(resp) => {
            let dt = Local::now();
            let text = resp.text();
            println!("{},登录结果: {:?}", dt, text);
            // println!("{},登录成功", dt);
        }
        Err(_) => {
            println!("登录失败");
        }
    }
    ()
}

fn sign_in(client: &Client) {
    let mut bypasser = {
        cloudflare_bypasser::Bypasser::default()
            .retry(30)
            .random_user_agent(true)
            .user_agent("Mozilla/5.0")
            .wait(5)
    };
    let (cookie, user_agent);
    loop {
        if let Ok((c, ua)) =  bypasser.bypass("https://www.hdarea.co") {
            cookie = c;
            user_agent = ua;
            break;
        }
    }
    let mut params = HashMap::new();
    params.insert("action", "sign_in");
    let resp = client
        .post("https://www.hdarea.co/sign_in.php")
        .header(reqwest::header::COOKIE, cookie)
        .header(reqwest::header::USER_AGENT, user_agent)
        .form(&params)
        .send();
    match resp {
        Ok(resp) => {
            let text = resp.text();
            let dt = Local::now();
            println!("{},签到结果: {:?}", dt, text);
        }
        Err(_) => {
            println!("签到失败");
        }
    }
}
