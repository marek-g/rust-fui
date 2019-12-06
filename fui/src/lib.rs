extern crate drawing;
extern crate drawing_gl;
extern crate failure;
extern crate find_folder;
extern crate typed_builder;
extern crate typemap;
extern crate winit;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub mod application;

mod children_source;
pub use children_source::*;

mod common;
pub use common::*;

mod control;
pub use control::*;

mod control_object;
pub use control_object::*;

mod drawing_context;
pub use drawing_context::*;

mod events;
pub use events::*;

mod high_dpi;
pub use high_dpi::*;

mod observable;
pub use observable::*;

mod style;
pub use style::*;

mod threading;
pub use threading::*;

mod view;
pub use view::*;

mod window;
pub use window::*;

mod window_manager;
pub use window_manager::*;
