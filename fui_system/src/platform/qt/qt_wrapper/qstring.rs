use crate::platform::qt::qt_wrapper::QByteArray;
use crate::FUISystemError;
use std::path::PathBuf;

pub struct QString {
    pub this: *mut ::std::os::raw::c_void,
    pub is_owned: bool,
}

impl QString {
    pub fn null() -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QString_null();
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(Self {
                this,
                is_owned: true,
            })
        }
    }

    pub fn from_str(text: &str) -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QString_fromUtf8(
                text.as_ptr() as *const i8,
                text.len() as i32,
            );
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(Self {
                this,
                is_owned: false,
            })
        }
    }

    pub fn as_utf8(&self) -> Result<QByteArray, FUISystemError> {
        unsafe {
            let qbytearray = crate::platform::qt::qt_wrapper::QString_toUtf8(self.this);
            if qbytearray.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(QByteArray { this: qbytearray })
        }
    }

    pub fn as_string(&self) -> Result<String, FUISystemError> {
        let utf8 = self.as_utf8()?;
        let utf8 = utf8.as_bytes();
        Ok(String::from_utf8_lossy(utf8).into_owned())
    }

    pub fn as_path_buf(&self) -> Result<PathBuf, FUISystemError> {
        Ok(PathBuf::from(self.as_string()?))
    }
}

impl Drop for QString {
    fn drop(&mut self) {
        if self.is_owned {
            unsafe {
                crate::platform::qt::qt_wrapper::QString_delete(self.this);
            }
        }
    }
}
