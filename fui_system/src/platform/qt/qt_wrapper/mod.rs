#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use qaction::*;
pub use qapplication::*;
pub use qicon::*;
pub use qmenu::*;
pub use qopenglcontext::*;
pub use qpixmap::*;
pub use qslot::*;
pub use qstring::*;
pub use qsystemtrayicon::*;
pub use qwindow::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod qaction;
mod qapplication;
mod qicon;
mod qmenu;
mod qopenglcontext;
mod qpixmap;
mod qslot;
mod qstring;
mod qsystemtrayicon;
mod qwindow;
