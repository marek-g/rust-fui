use crate::platform::qt::qt_wrapper::callback_helper::{
    callback_to_pointer, callback_trampoline, RawCallback,
};
use crate::platform::qt::qt_wrapper::QString;
use std::ffi::c_void;

pub struct QWindow {
    pub this: *mut ::std::os::raw::c_void,

    initialize_gl_callback: Option<Box<dyn Drop>>,
    paint_gl_callback: Option<Box<dyn Drop>>,
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
            let raw_callback = RawCallback::new(callback);
            crate::platform::qt::qt_wrapper::QWindow_setInitializeGLFunc(
                self.this,
                Some(callback_trampoline::<F>),
                raw_callback.ptr,
            );
            self.initialize_gl_callback = Some(Box::new(raw_callback));
        }
    }

    pub fn set_paint_gl_callback<F: 'static + FnMut()>(&mut self, callback: F) {
        unsafe {
            let raw_callback = RawCallback::new(callback);
            crate::platform::qt::qt_wrapper::QWindow_setPaintGLFunc(
                self.this,
                Some(callback_trampoline::<F>),
                raw_callback.ptr,
            );
            self.paint_gl_callback = Some(Box::new(raw_callback));
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
