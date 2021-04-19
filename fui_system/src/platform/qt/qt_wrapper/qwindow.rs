use crate::platform::qt::qt_wrapper::callback_helper::{callback_to_pointer, callback_trampoline};
use crate::platform::qt::qt_wrapper::QString;
use std::ffi::c_void;

pub struct QWindow {
    pub this: *mut ::std::os::raw::c_void,

    initialize_gl_callback: Option<Box<dyn 'static + FnMut()>>,
    paint_gl_callback: Option<Box<dyn 'static + FnMut()>>,
}

impl QWindow {
    pub fn new(parent: Option<&mut QWindow>) -> Result<Self, ()> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QWindow_new(
                parent.map_or(0 as *mut ::std::os::raw::c_void, |p| p.this),
            );
            if this.is_null() {
                return Err(());
            }

            Ok(Self {
                this,
                initialize_gl_callback: None,
                paint_gl_callback: None,
            })
        }
    }

    pub fn set_title(&mut self, text: &QString) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_setTitle(self.this, text.this);
        }
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_setVisible(
                self.this,
                if is_visible { 1 } else { 0 },
            );
        }
    }

    pub fn set_initialize_gl_callback<F: 'static + FnMut()>(&mut self, callback: F) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_setInitializeGLFunc(
                self.this,
                Some(callback_trampoline::<F>),
                callback_to_pointer(callback),
            );
        }
    }

    pub fn set_paint_gl_callback<F: 'static + FnMut()>(&mut self, callback: F) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_setPaintGLFunc(
                self.this,
                Some(callback_trampoline::<F>),
                callback_to_pointer(callback),
            );
        }
    }
}

impl Drop for QWindow {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_delete(self.this);
        }
    }
}
