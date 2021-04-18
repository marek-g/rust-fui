use thiserror::Error;

pub use common::SystemMenuItem;

pub use platform::SystemApplication;
pub use platform::SystemMessageIcon;
pub use platform::SystemTray;
pub use platform::SystemWindow;

mod common;
mod platform;

#[derive(Error, Debug)]
pub enum TrayError {
    #[error("OsError: {0}")]
    OsError(String),
}
