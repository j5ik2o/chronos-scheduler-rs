# chronos-scheduler-rs

[![Workflow Status](https://github.com/j5ik2o/chronos-scheduler-rs/workflows/ci/badge.svg)](https://github.com/j5ik2o/chronos-scheduler-rs/actions?query=workflow%3A%22ci%22)
[![crates.io](https://img.shields.io/crates/v/chronos-scheduler-rs.svg)](https://crates.io/crates/chronos-scheduler-rs)
[![docs.rs](https://docs.rs/chronos-scheduler-rs/badge.svg)](https://docs.rs/chronos-scheduler-rs)
[![Renovate](https://img.shields.io/badge/renovate-enabled-brightgreen.svg)](https://renovatebot.com)
[![dependency status](https://deps.rs/repo/github/j5ik2o/chronos-scheduler-rs/status.svg)](https://deps.rs/repo/github/j5ik2o/chronos-scheduler-rs)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-APACHE2.0-blue.svg)](https://opensource.org/licenses/apache-2-0)
[![](https://tokei.rs/b1/github/j5ik2o/chronos-scheduler-rs)](https://github.com/XAMPPRocky/tokei)

A Rust crate for Job Scheduler. This Job Scheduler is simple.

## Installation

Add the following configuration to `Cargo.toml`.

```toml
[dependencies]
chronos-scheduler-rs = "1.0.XXX"
```

## Usage

```rust
// Create a new job scheduler
let mut job_scheduler = JobScheduler::new();

// Set the interval for the scheduler's tick to 1 minute
let tick_interval = Duration::minutes(1);
// Initialize a counter to track the number of job executions
let mut counter = 0;

// Define a new job that runs every minute
let job = Job::new(
  "*/1 * * * *".to_string(), // Cron expression for every minute
  |job_context| {
    // Borrow the data passed to the job context and unwrap it
    let data = job_context.data().borrow().unwrap();
    // Log the details of the job execution including the schedule, current time, counter, and data
    log::debug!(
      "schedule_datetime = {}, now = {}: {}) {}",
      job_context.trigger(), // The scheduled datetime
      job_context.now(),     // The current datetime
      counter,               // The execution counter
      data                   // The data passed to the job context
    );
    // Increment the counter after each job execution
    counter += 1;
  },
  Some("Hello, world!"), // Optional data passed to the job context
);
// Add the defined job to the job scheduler
job_scheduler.add_job(job);

// Enter an infinite loop to continuously check and run scheduled jobs
loop {
  // Check and execute jobs based on the current datetime
  job_scheduler.tick();
  // Log the waiting period until the next tick
  log::debug!("waiting for {} seconds...", tick_interval.num_seconds());
  // Sleep for the duration of the tick interval
  sleep(tick_interval.to_std().unwrap());
}
```

## Related Crates

- [j5ik2o/chronos-parser-rs](https://github.com/j5ik2o/chronos-parser-rs)

## Implementations for Scala

- [j5ik2o/chronos-scheduler-scala](https://github.com/j5ik2o/chronos-scheduler-scala)
- [j5ik2o/chronos-parser-scala](https://github.com/j5ik2o/chronos-parser-scala)
