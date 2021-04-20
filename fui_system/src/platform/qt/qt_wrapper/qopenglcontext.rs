use std::ffi::{c_void, CString, NulError};

pub struct QOpenGLContext {
    pub this: *mut ::std::os::raw::c_void,
    pub is_owned: bool,
}

impl QOpenGLContext {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QOpenGLContext_new();
            if this.is_null() {
                return Err(());
            }

            Ok(Self {
                this,
                is_owned: true,
            })
        }
    }

    pub fn get_proc_address(&self, proc_name: &str) -> Result<*const c_void, ()> {
        unsafe {
            let c_str = CString::new(proc_name).map_err(|e| ())?;
            let addr = crate::platform::qt::qt_wrapper::QOpenGLContext_getProcAddress(
                self.this,
                c_str.as_ptr(),
            );
            if let Some(addr) = addr {
                Ok(addr as *const c_void)
            } else {
                Err(())
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
