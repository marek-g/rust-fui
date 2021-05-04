use crate::common::callback_helper::{callback_to_pointer, callback_trampoline, RawCallback};
use crate::platform::qt::qt_wrapper::{QOpenGLContext, QString};
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

    pub fn get_width(&mut self) -> i32 {
        unsafe { crate::platform::qt::qt_wrapper::QWindow_getWidth(self.this) }
    }

    pub fn get_height(&mut self) -> i32 {
        unsafe { crate::platform::qt::qt_wrapper::QWindow_getHeight(self.this) }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_resize(self.this, width, height);
        }
    }

    pub fn update(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_update(self.this);
        }
    }

    pub fn on_initialize_gl<F: 'static + FnMut()>(&mut self, callback: F) {
        unsafe {
            let raw_callback = RawCallback::new(callback);
            crate::platform::qt::qt_wrapper::QWindow_setInitializeGLFunc(
                self.this,
                Some(raw_callback.get_trampoline_func()),
                raw_callback.get_trampoline_func_data(),
            );
            self.initialize_gl_callback = Some(Box::new(raw_callback));
        }
    }

    pub fn on_paint_gl<F: 'static + FnMut()>(&mut self, callback: F) {
        unsafe {
            let raw_callback = RawCallback::new(callback);
            crate::platform::qt::qt_wrapper::QWindow_setPaintGLFunc(
                self.this,
                Some(raw_callback.get_trampoline_func()),
                raw_callback.get_trampoline_func_data(),
            );
            self.paint_gl_callback = Some(Box::new(raw_callback));
        }
    }

    pub fn get_context(&self) -> Result<QOpenGLContext, ()> {
        unsafe {
            let context_this = crate::platform::qt::qt_wrapper::QWindow_context(self.this);
            if context_this.is_null() {
                return Err(());
            }

            Ok(QOpenGLContext {
                this: context_this,
                is_owned: false,
            })
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
