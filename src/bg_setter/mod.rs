pub mod windows;

pub type Result<T> = core::result::Result<T, Error>;

/// Behavior changing the desktop background of the system 
pub trait BackgroundSetter {
    /// Change system background
    fn set_background(&self, image_path: &str) -> Result<()>;
}

#[derive(Debug)]
/// Errors of module bg_setter
pub enum Error{
}