use thiserror::Error;

pub use common::MenuItem;

pub use platform::Application;
pub use platform::TrayIcon;
pub use platform::TrayIconType;
pub use platform::Window;

mod common;
mod platform;

#[derive(Error, Debug)]
pub enum TrayError {
    #[error("OsError: {0}")]
    OsError(String),
}
