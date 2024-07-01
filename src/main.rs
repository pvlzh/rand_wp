mod configuration;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let config = configuration::init()?;

    Ok(())
}


/// Base error
#[derive(Debug)]
pub struct ApplicationError {
    pub message: String
}