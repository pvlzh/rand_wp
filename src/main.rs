mod configuration;
mod job;
mod bg_setter;

use job::{Job, JobRunner};

pub type Result<T> = core::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let config = configuration::init()?;

    let job = Printer;
    JobRunner::new(job, config.job)
        .run()
        .await?;

    Ok(())
}


pub struct Printer;

impl Job for Printer{
    async fn execute(&self) -> job::Result<()>{
        println!("Hello every 10 sec");
        Ok(())
    }
}

/// Application error
#[derive(Debug)]
pub enum Error {
    ConfigurationError(configuration::Error),
    BackgroundJobExecutionError(job::Error),
}

impl From<configuration::Error> for Error {
    fn from(error: configuration::Error) -> Self {
        Self::ConfigurationError(error)
    }
}

impl From<job::Error> for Error {
    fn from(error: job::Error) -> Self {
        Self::BackgroundJobExecutionError(error)
    }
}
