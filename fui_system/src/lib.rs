use thiserror::Error;

pub use common::MenuItem;

pub use platform::Application;
pub use platform::ApplicationOptions;
pub use platform::Dispatcher;
pub use platform::Edge;
pub use platform::FileDialog;
pub use platform::Icon;
pub use platform::TranslucentEffect;
pub use platform::TrayIcon;
pub use platform::TrayIconType;
pub use platform::Window;
pub use platform::WindowFrameType;

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
