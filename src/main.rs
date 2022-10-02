extern crate job_scheduler;
use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();

    // 0 0 9 * * * *
    sched.add(Job::new("0/5 * * * * * *".parse().unwrap(), || {
        let client = reqwest::Client::builder()
            .user_agent("sss")
            .build()
            .unwrap();
        let resp = client.get("https://baidu.com/").send().await.unwrap();
        println!("{:#?}", resp);
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_secs(5));
    }
}
