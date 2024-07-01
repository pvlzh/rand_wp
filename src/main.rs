mod configuration;
mod job;

use job::{Job, JobExecutionError, JobRunner};

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let config = configuration::init()?;

    let job = Printer;
    let runner = JobRunner::new(job, config.job);

    _ = runner.run().await;

    Ok(())
}


pub struct Printer;

impl Job for Printer{
    async fn execute(&self) -> Result<(), JobExecutionError>{
        println!("Hello every 10 sec");
        Ok(())
    }
}

/// Base error
#[derive(Debug)]
pub struct ApplicationError(String);