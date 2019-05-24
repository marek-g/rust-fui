extern crate drawing;
extern crate drawing_gl;
extern crate failure;
extern crate find_folder;
extern crate winit;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub mod application;

pub mod common;

mod control;
pub use control::*;

mod control_object;
pub use control_object::*;

pub mod controls;

mod drawing_context;
pub use drawing_context::*;

pub mod events;
pub mod high_dpi;
pub mod layout;

mod observable;
pub use observable::*;

mod threading;
pub use threading::*;

mod view;
pub use view::*;

mod window;
pub use window::*;

mod window_manager;
pub use window_manager::*;
