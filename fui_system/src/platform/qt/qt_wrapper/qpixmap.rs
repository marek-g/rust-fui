use crate::FUISystemError;

pub struct QPixmap {
    pub this: *mut ::std::os::raw::c_void,
}

impl QPixmap {
    pub fn from_data(data: &[u8]) -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QPixmap_new();
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            if crate::platform::qt::qt_wrapper::QPixmap_loadFromData(
                this,
                data.as_ptr(),
                data.len() as i32,
            ) == 0
            {
                return Err(FUISystemError::OsError(
                    "Cannot load pixmap from data.".to_string(),
                ));
            }

            Ok(Self { this })
        }
    }
}

impl Drop for QPixmap {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QPixmap_delete(self.this);
        }
    }
}
