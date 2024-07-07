use std::path::Path;
use tokio::{fs::{self, File}, io::AsyncWriteExt};


pub mod goodfon;

pub type Result<T> = core::result::Result<T, Error>;

/// Errors of the image provider
#[derive(Debug)]
pub enum Error {
    GoodFonProviderError(goodfon::Error)
}

/// Image provider behavior
pub trait ImageProvider {
    /// Get an image
    async fn get_image(&self) -> Result<ImageBytes>; 
}

pub struct ImageBytes(Vec<u8>);

impl ImageBytes {
    /// Save image data in the specified path
    pub async fn save(&self, path: &str) -> core::result::Result<(), std::io::Error> {
        let path = Path::new(path);
        if path.exists() {
            fs::remove_file(path).await?;
        }

        let mut file = File::create(path).await?;
        file.write_all(&self.0).await?;

        Ok(())
    }
}

impl From<Vec<u8>> for ImageBytes {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}