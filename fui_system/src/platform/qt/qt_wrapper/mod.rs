#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub(crate) use ffi_event::*;
pub use qaction::*;
pub use qapplication::*;
pub use qbytearray::*;
pub use qfiledialog::*;
pub use qicon::*;
pub use qmenu::*;
pub use qopenglcontext::*;
pub use qpixmap::*;
pub use qslot::*;
pub use qstring::*;
pub use qstringlist::*;
pub use qsurfaceformat::*;
pub use qsystemtrayicon::*;
pub use qwindow::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod ffi_event;
mod qaction;
mod qapplication;
mod qbytearray;
mod qfiledialog;
mod qicon;
mod qmenu;
mod qopenglcontext;
mod qpixmap;
mod qslot;
mod qstring;
mod qstringlist;
mod qsurfaceformat;
mod qsystemtrayicon;
mod qwindow;
