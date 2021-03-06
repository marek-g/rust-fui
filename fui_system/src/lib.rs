use thiserror::Error;

pub use common::MenuItem;

pub use platform::Application;
pub use platform::ApplicationOptionsBuilder;
pub use platform::Icon;
pub use platform::TrayIcon;
pub use platform::TrayIconType;
pub use platform::Window;

mod common;
mod platform;

#[derive(Error, Debug)]
pub enum FUISystemError {
    #[error("Not initialized")]
    NotInitialized,

    #[error("Out of memory")]
    OutOfMemory,

    #[error("OsError: {0}")]
    OsError(String),
}
