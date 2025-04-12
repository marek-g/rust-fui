use crate::common::callback_helper::RawCallback;
use crate::platform::qt::qt_wrapper::QString;
use crate::FUISystemError;
use std::ffi::CString;
use std::os::raw::c_char;

pub enum QApplicationAttribute {
    ShareOpenGLContexts,
}

impl QApplicationAttribute {
    pub fn to_i32(&self) -> i32 {
        match self {
            QApplicationAttribute::ShareOpenGLContexts => 18,
        }
    }
}

pub struct QApplication {
    pub this: *mut ::std::os::raw::c_void,
}

impl QApplication {
    pub fn new() -> Result<Self, FUISystemError> {
        unsafe {
            // convert args() to argc, argv
            let mut args = std::env::args()
                .map(|arg| CString::new(arg).unwrap())
                .collect::<Vec<CString>>();

            // run fui apps in XWayland because of transparency glitches
            // see: wayland.md
            if cfg!(target_family = "unix")
                && std::env::args().find(|el| el == "--platform").is_none()
            {
                args.push(CString::new("--platform").unwrap());
                args.push(CString::new("xcb").unwrap());
            }

            let c_args = args
                .iter()
                .map(|arg| arg.as_ptr())
                .collect::<Vec<*const c_char>>();

            let this = crate::platform::qt::qt_wrapper::QApplication_new(
                c_args.len() as i32,
                c_args.as_ptr() as *mut *const i8,
            );
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(Self { this })
        }
    }

    pub fn set_attribute(attr: QApplicationAttribute, enable: bool) {
        unsafe {
            crate::platform::qt::qt_wrapper::QApplication_setAttribute(
                attr.to_i32(),
                if enable { 1 } else { 0 },
            );
        }
    }

    pub fn set_application_display_name(text: &QString) {
        unsafe {
            crate::platform::qt::qt_wrapper::QApplication_setApplicationDisplayName(text.this);
        }
    }

    pub fn exec() -> i32 {
        unsafe { crate::platform::qt::qt_wrapper::QApplication_exec() }
    }

    pub fn exit(result_code: i32) {
        unsafe {
            crate::platform::qt::qt_wrapper::QApplication_exit(result_code);
        }
    }

    pub fn is_gui_thread() -> bool {
        unsafe { crate::platform::qt::qt_wrapper::QApplication_isGuiThread() != 0 }
    }

    ///
    /// Posts function to be executed on the main event loop.
    /// This function can be called safety only from the QApplication thread.
    ///
    pub unsafe fn post_func_same_thread<F>(func: F)
    where
        F: FnOnce() + 'static,
    {
        // TODO: fix memory leak?
        let raw_callback = RawCallback::new_once(func);

        crate::platform::qt::qt_wrapper::QApplication_postFunc(
            Some(raw_callback.get_trampoline_func()),
            raw_callback.get_trampoline_func_data(),
        );

        std::mem::forget(raw_callback);
    }

    ///
    /// Posts function to be executed on the main event loop.
    /// This function can be called from any thread.
    ///
    pub fn post_func_any_thread<F>(func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe {
            // TODO: fix memory leak?
            let raw_callback = RawCallback::new_once(func);

            crate::platform::qt::qt_wrapper::QApplication_postFunc(
                Some(raw_callback.get_trampoline_func()),
                raw_callback.get_trampoline_func_data(),
            );

            std::mem::forget(raw_callback);
        }
    }

    #[allow(dead_code)]
    pub fn about_qt() {
        unsafe {
            crate::platform::qt::qt_wrapper::QApplication_aboutQt();
        }
    }
}

impl Drop for QApplication {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QApplication_delete(self.this);
        }
    }
}
