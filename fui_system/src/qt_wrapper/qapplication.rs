use crate::qt_wrapper::QString;
use std::ffi::CString;
use std::os::raw::c_char;

pub struct QApplication {
    pub this: *mut ::std::os::raw::c_void,
}

impl QApplication {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            // convert args() to argc, argv
            let args = std::env::args()
                .map(|arg| CString::new(arg).unwrap())
                .collect::<Vec<CString>>();
            let c_args = args
                .iter()
                .map(|arg| arg.as_ptr())
                .collect::<Vec<*const c_char>>();

            let this = crate::qt_wrapper::QApplication_new(
                c_args.len() as i32,
                c_args.as_ptr() as *mut *const i8,
            );
            if this.is_null() {
                return Err(());
            }

            Ok(Self { this })
        }
    }

    pub fn set_application_display_name(text: &QString) {
        unsafe {
            crate::qt_wrapper::QApplication_setApplicationDisplayName(text.this);
        }
    }

    pub fn exec() {
        unsafe {
            crate::qt_wrapper::QApplication_exec();
        }
    }

    pub fn exit(result_code: i32) {
        unsafe {
            crate::qt_wrapper::QApplication_exit(result_code);
        }
    }

    pub fn about_qt() {
        unsafe {
            crate::qt_wrapper::QApplication_aboutQt();
        }
    }
}

impl Drop for QApplication {
    fn drop(&mut self) {
        unsafe {
            crate::qt_wrapper::QApplication_delete(self.this);
        }
    }
}
