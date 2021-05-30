use crate::common::callback_helper::{RawCallback, RawCallbackWithParam};
use crate::platform::qt::qt_wrapper::ffi_event::FFIEvent;
use crate::platform::qt::qt_wrapper::{QIcon, QOpenGLContext, QString};
use crate::FUISystemError;
use std::ffi::c_void;

pub struct QWindow {
    pub this: *mut ::std::os::raw::c_void,

    event_callback: Option<Box<dyn Drop>>,
    initialize_gl_callback: Option<Box<dyn Drop>>,
    paint_gl_callback: Option<Box<dyn Drop>>,
}

impl QWindow {
    pub fn new(parent: Option<&mut QWindow>) -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QWindow_new(
                parent.map_or(0 as *mut ::std::os::raw::c_void, |p| p.this),
            );
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(Self {
                this,
                event_callback: None,
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

    pub fn set_icon(&mut self, icon: &QIcon) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_setIcon(self.this, icon.this);
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

    pub fn set_minimum_size(&mut self, width: i32, height: i32) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_setMinimumSize(self.this, width, height);
        }
    }

    pub fn update(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_update(self.this);
        }
    }

    pub fn on_event<F: 'static + FnMut(&FFIEvent) -> bool>(&mut self, mut callback: F) {
        unsafe {
            let raw_callback = RawCallbackWithParam::new(move |ptr| {
                let event = &*(ptr as *const FFIEvent);
                if callback(event) {
                    1 as *mut c_void
                } else {
                    0 as *mut c_void
                }
            });

            crate::platform::qt::qt_wrapper::QWindow_setEventFunc(
                self.this,
                Some(raw_callback.get_trampoline_func()),
                raw_callback.get_trampoline_func_data(),
            );
            self.event_callback = Some(Box::new(raw_callback));
        }
    }

    ///
    /// Warning! This method on Windows is called from QWindow::create()
    /// and on Linux from message loop - QApplication::exec().
    /// It makes it harder to use reference to Window inside
    /// the callback on Windows.
    /// It may be safer to initialize gl on the first paintGL callback.
    ///
    #[allow(dead_code)]
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

    pub fn get_context(&self) -> Result<QOpenGLContext, FUISystemError> {
        unsafe {
            let context_this = crate::platform::qt::qt_wrapper::QWindow_context(self.this);
            if context_this.is_null() {
                return Err(FUISystemError::NotInitialized);
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
