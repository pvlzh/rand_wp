#![windows_subsystem = "windows"]

mod configuration;
mod job;
mod wp_setter;
mod rand_img;

use std::env;

use job::{Job, JobRunner};
use rand_img::{goodfon::GoodFonProvider, ImageProvider};
use wp_setter::{windows::WindowsWallpaperSetter, WallpaperSetter};

pub type Result<T> = core::result::Result<T, Error>;

/// Application error
#[derive(Debug)]
pub enum Error {
    ConfigurationError(configuration::Error),
    BackgroundJobExecutionError(job::Error),
}
#[tokio::main]
async fn main() -> Result<()> {
    let config = configuration::init().await?;

    let img_provider = GoodFonProvider::new(config.image);
    let wp_setter = WindowsWallpaperSetter;

    let job = WallpaperChanger::new(img_provider, wp_setter);
    JobRunner::new(job, config.job)
        .run()
        .await?;

    Ok(())
}


/// Job of changing the desktop wallpaper
struct WallpaperChanger<IP: ImageProvider, WS: WallpaperSetter> {
    img_provider: IP,
    wp_setter: WS
}

impl <IP: ImageProvider, WS: WallpaperSetter> WallpaperChanger<IP, WS> {
    /// Initializing the job of changing the desktop wallpaper
    pub fn new(img_provider: IP, wp_setter: WS) -> Self{
        Self { img_provider, wp_setter }
    }
}

impl<IP: ImageProvider, WS: WallpaperSetter> Job for WallpaperChanger<IP, WS> {
    /// Change the wallpaper
    async fn execute(&self) -> job::Result<()> {
        let image_provider = &self.img_provider;
        let wallpaper_setter = &self.wp_setter;

        let temp_dir = env::temp_dir();
        let current_image_path = format!("{0}/current_wallpaper.jpg", temp_dir.display());

        let image_bytes = image_provider.get_image().await?;
        
        if let Err(error) = image_bytes.save(&current_image_path).await {
            return Err(job::Error::ImageSavingError(error));
        }

        wallpaper_setter.set_background(&current_image_path)?;
        Ok(())
    }
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

impl From<rand_img::Error> for job::Error {
    fn from(error: rand_img::Error) -> Self {
        Self::ImageReceivingError(error)
    }
}

impl From<wp_setter::Error> for job::Error {
    fn from(error: wp_setter::Error) -> Self {
        Self::WallpaperChangeError(error)
    }
}