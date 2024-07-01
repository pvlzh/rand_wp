use super::{WallpaperSetter, Result};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use winapi::um::winuser::{
    SystemParametersInfoW, 
    SPIF_SENDCHANGE, 
    SPIF_UPDATEINIFILE,
    SPI_SETDESKWALLPAPER,
};

pub struct WindowsWallpaperSetter;

/// Behavior changing the desktop background of the windows system
impl WallpaperSetter for WindowsWallpaperSetter {
    /// Change the background image of the windows system
    fn set_background(&self, image_path: &str) -> Result<()> {
        let mut path: Vec<u16> = OsStr::new(image_path)
            .encode_wide()
            .collect();

        // append null byte
        path.push(0);

        let ui_action = SPI_SETDESKWALLPAPER;
        let ui_param = 0;
        let pv_param = path.as_ptr() as *mut c_void;
        let f_win_ini = SPIF_UPDATEINIFILE | SPIF_SENDCHANGE;

        let _ = unsafe {
            SystemParametersInfoW(
                ui_action, 
                ui_param, 
                pv_param, 
                f_win_ini) == 1
        };
        
        Ok(())
    }
}