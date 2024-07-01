use std::{fs::{self, File}, io::Write as _, path::Path};

pub mod goodfon;

pub type Result<T> = core::result::Result<T, Error>;

/// Error of job execution
#[derive(Debug)]
pub enum Error {
    GoodFonProviderError(goodfon::Error)
}

/// 
pub trait ImageProvider {
    ///
    async fn get_image(&self) -> Result<ImageBytes>; 
}

pub struct ImageBytes(Vec<u8>);

impl ImageBytes {

    ///
    pub fn save(&self, path: &str) -> core::result::Result<(), std::io::Error> {
        let path = Path::new(path);
        if path.exists() {
            fs::remove_file(path)?;
        }

        let mut file = File::create(path)?; // todo: tokio async
        file.write_all(&self.0)?;

        Ok(())
    }
}

impl From<Vec<u8>> for ImageBytes {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}