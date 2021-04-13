use thiserror::Error;

pub use platform::SystemApplication;
pub use platform::SystemMessageIcon;
pub use platform::SystemTray;
pub use platform::SystemWindow;

mod platform;

#[derive(Error, Debug)]
pub enum TrayError {
    #[error("OsError: {0}")]
    OsError(String),
}
