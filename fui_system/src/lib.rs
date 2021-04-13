mod platform;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrayError {
    #[error("OsError: {0}")]
    OsError(String),
}

pub use platform::SystemApplication;
pub use platform::SystemMessageIcon;
pub use platform::SystemTray;
pub use platform::SystemWindow;

#[cfg(target_os = "linux")]
mod qt_wrapper;
