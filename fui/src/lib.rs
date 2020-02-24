pub type Result<T> = std::result::Result<T, failure::Error>;

mod children_source;
pub use children_source::*;

mod common;
pub use common::*;

mod control;
pub use control::*;

mod resources;
pub use resources::*;

mod events;
pub use events::*;

mod observable;
pub use observable::*;

mod style;
pub use style::*;

mod view;
pub use view::*;
