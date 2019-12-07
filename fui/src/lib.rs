extern crate drawing;
extern crate failure;
extern crate typed_builder;
extern crate typemap;
extern crate winit;

pub type Result<T> = std::result::Result<T, failure::Error>;

mod children_source;
pub use children_source::*;

mod common;
pub use common::*;

mod control;
pub use control::*;

mod control_object;
pub use control_object::*;

mod resources;
pub use resources::*;

mod events;
pub use events::*;

mod observable;
pub use observable::*;

mod style;
pub use style::*;

mod threading;
pub use threading::*;

mod view;
pub use view::*;
