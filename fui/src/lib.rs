pub type Result<T> = std::result::Result<T, failure::Error>;

mod children_source;
pub use children_source::*;

mod common;
pub use common::*;

mod control;
pub use control::*;

mod drawing;
pub use crate::drawing::*;

mod events;
pub use events::*;

mod observable;
pub use observable::*;

mod services;
pub use services::*;

mod style;
pub use style::*;

mod view;
pub use view::*;
