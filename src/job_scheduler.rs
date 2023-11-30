use ulid_generator_rs::{ULIDGenerator, ULID};

use crate::job::{Job, JobContext};

pub struct JobScheduler<F, T>
where
  F: FnMut(JobContext<T>), {
  id: ULID,
  jobs: Vec<Job<F, T>>,
}

impl<F, T> JobScheduler<F, T>
where
  F: FnMut(JobContext<T>),
{
  pub fn new() -> Self {
    let mut generator = ULIDGenerator::new();
    let id = generator.generate().unwrap();
    JobScheduler { id, jobs: Vec::new() }
  }

  pub fn add_job(&mut self, job: Job<F, T>) {
    self.jobs.push(job);
  }

  pub fn tick(&mut self) {
    for job in self.jobs.iter_mut() {
      job.tick();
    }
  }
}

#[cfg(test)]
mod tests {
  use std::env;
  use std::thread::sleep;

  use chrono::{Duration, Utc};

  use super::*;

  #[test]
  fn test_tick() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let mut job_scheduler = JobScheduler::new();

    let tick_interval = Duration::seconds(1);
    let mut counter = 0;

    let job = Job::new(
      "*/1 * * * *".to_string(),
      |job_context| {
        let str = job_context.data().borrow().unwrap();
        log::debug!(
          "schedule_datetime = {}, now = {}: {}) {}",
          job_context.trigger(),
          job_context.now(),
          counter,
          str
        );
        counter += 1;
      },
      Some("Hello, world!"),
    )
    .with_tick_interval(tick_interval);
    job_scheduler.add_job(job);

    let now = Utc::now();
    let end_time = now + Duration::seconds(2);

    while Utc::now() < end_time {
      log::debug!("tick: now = {}", Utc::now());
      job_scheduler.tick();
      log::debug!("waiting for {} seconds...", tick_interval.num_seconds());
      sleep(tick_interval.to_std().unwrap());
    }

    assert_eq!(counter, 1);
  }
}
