use std::time::Duration;
use tokio::time::sleep;
use crate::configuration::JobConfig;

pub type Result<T> = core::result::Result<T, Error>;

/// Error of job execution
#[derive(Debug)]
pub enum Error {
}


/// Job handler trait
pub trait Job {
    async fn execute(&self) -> Result<()>;
}

/// Jobs runner
pub struct JobRunner<TJob> where TJob: Job  {
    job: TJob,
    config: JobConfig,
}

impl<TJob: Job> JobRunner<TJob> {
    /// Create new background job
    pub fn new(job: TJob, config: JobConfig) -> JobRunner<TJob> {
        Self { job, config }
    }

    /// Run background job
    pub async fn run(&self) -> Result<()> {
        loop {
            self.job.execute().await?; // todo: write into log and continue
            sleep(Duration::from_secs(self.config.interval_sec)).await;
        }
    }
}
