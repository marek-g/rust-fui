use crate::FUISystemError;
use std::ffi::{c_void, CString};

pub struct QOpenGLContext {
    pub this: *mut ::std::os::raw::c_void,
    pub is_owned: bool,
}

impl QOpenGLContext {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QOpenGLContext_new();
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(Self {
                this,
                is_owned: true,
            })
        }
    }

    pub fn get_proc_address(&self, proc_name: &str) -> Result<*const c_void, FUISystemError> {
        unsafe {
            let c_str = CString::new(proc_name).map_err(|_| {
                FUISystemError::OsError("Null error for OpenGL procedure address.".to_string())
            })?;
            let addr = crate::platform::qt::qt_wrapper::QOpenGLContext_getProcAddress(
                self.this,
                c_str.as_ptr(),
            );
            if let Some(addr) = addr {
                Ok(addr as *const c_void)
            } else {
                Err(FUISystemError::OsError(format!(
                    "Cannot find OpenGL procedure address: {}",
                    proc_name
                )))
            }
        }
    }
}

impl Drop for QOpenGLContext {
    fn drop(&mut self) {
        if self.is_owned {
            unsafe {
                crate::platform::qt::qt_wrapper::QOpenGLContext_delete(self.this);
            }
        }
    }
}
