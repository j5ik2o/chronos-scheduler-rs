use chrono::{DateTime, Duration, Utc};
use chronos_parser_rs::CronSchedule;
use std::cell::RefCell;
use std::rc::Rc;
use ulid_generator_rs::{ULIDGenerator, ULID};

#[derive(Debug, Clone)]
pub struct Job<F, T>
where
    F: FnMut(JobContext<T>),
{
    id: ULID,
    crond_expr: String,
    tick_interval: Duration,
    limit_missed_runs: usize,
    func: F,
    data: Rc<RefCell<Option<T>>>,
    cond_schedule: CronSchedule<Utc>,
    last_tick: Option<DateTime<Utc>>,
    _phantom: std::marker::PhantomData<T>,
}

pub struct JobContext<'a, T> {
    cron_expr: String,
    trigger: &'a DateTime<Utc>,
    now: &'a DateTime<Utc>,
    data: Rc<RefCell<Option<T>>>,
}

impl<'a, T> JobContext<'a, T> {
    pub fn new(
        cron_expr: &str,
        trigger: &'a DateTime<Utc>,
        now: &'a DateTime<Utc>,
        data: Rc<RefCell<Option<T>>>,
    ) -> Self {
        JobContext {
            cron_expr: cron_expr.to_string(),
            trigger,
            now,
            data,
        }
    }

    pub fn cron_expr(&self) -> &str {
        &self.cron_expr
    }

    pub fn trigger(&self) -> &DateTime<Utc> {
        self.trigger
    }

    pub fn now(&self) -> &DateTime<Utc> {
        self.now
    }

    pub fn data(&self) -> Rc<RefCell<Option<T>>> {
        self.data.clone()
    }
}

impl<F, T> Job<F, T>
where
    F: FnMut(JobContext<T>),
{
    pub fn new(crond_expr: String, func: F, data: Option<T>) -> Self {
        let mut generator = ULIDGenerator::new();
        let id = generator.generate().unwrap();
        let cond_schedule = CronSchedule::new(&crond_expr).unwrap();
        Job {
            id,
            crond_expr,
            tick_interval: Duration::minutes(1),
            limit_missed_runs: 5,
            func,
            data: Rc::new(RefCell::new(data)),
            cond_schedule,
            last_tick: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn tick_interval(&self) -> &Duration {
        &self.tick_interval
    }

    pub fn with_tick_interval(mut self, tick_interval: Duration) -> Self {
        self.tick_interval = tick_interval;
        self
    }

    pub fn tick(&mut self) {
        let now = Utc::now();
        match self.last_tick {
            None => {
                self.last_tick = Some(now);
            }
            Some(lt) if lt + self.tick_interval < now => {
                let itr = self.cond_schedule.upcoming(lt).take(self.limit_missed_runs);
                for next_trigger in itr {
                    if next_trigger > now {
                        self.run(JobContext::new(
                            &self.crond_expr,
                            &next_trigger,
                            &now,
                            self.data.clone(),
                        ));
                        break;
                    }
                }
                self.last_tick = Some(now);
            }
            _ => {}
        }
    }

    pub fn run(&mut self, job_context: JobContext<T>) {
        (self.func)(job_context);
    }
}
