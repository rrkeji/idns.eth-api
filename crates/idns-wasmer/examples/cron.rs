extern crate cronjob;
use cronjob::CronJob;
use std::{thread, time};

fn main() {
    // Create the `CronJob` object.
    let mut cron = CronJob::new("Test Cron Threaded", on_cron);
    // Set seconds.
    cron.seconds("*");
    // Set minutes.
    // cron.minutes("0");
    // Start the cronjob.
    CronJob::start_job_threaded(cron);

    let ten_millis = time::Duration::from_millis(100000000000);
    thread::sleep(ten_millis);
}

// Our cronjob handler.
fn on_cron(name: &str) {
    println!("{}: It's time!", name);
}
