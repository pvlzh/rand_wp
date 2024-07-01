use std::time::Duration;
use tokio::time::sleep;
use crate::{configuration::JobConfig, ApplicationError};

/// Job handler trait
pub trait Job {
    async fn execute(&self) -> Result<(), JobExecutionError>;
}

/// Error of job execution
#[derive(Debug)]
pub struct JobExecutionError(String);

/// Convert JobExecutionError into ApplicationError
impl From<JobExecutionError> for ApplicationError{
    fn from(error: JobExecutionError) -> Self {
        Self(error.0.to_string())
    }
}

/// Jobs runner
pub struct JobRunner<TJob> where TJob: Job  {
    config: JobConfig,
    job: TJob,
}

impl<TJob: Job> JobRunner<TJob> {
    /// Create new background job
    pub fn new(job: TJob, config: JobConfig) -> JobRunner<TJob> {
        Self { job, config }
    }

    /// Run background job
    pub async fn run(&self) -> Result<(), JobExecutionError> {
        loop {
            self.job.execute().await?; // todo: write into log and continue
            sleep(Duration::from_secs(self.config.interval_sec)).await;
        }
    }
}