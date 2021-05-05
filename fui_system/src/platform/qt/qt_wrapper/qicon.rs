use crate::platform::qt::qt_wrapper::QPixmap;
use crate::FUISystemError;

pub struct QIcon {
    pub this: *mut ::std::os::raw::c_void,
}

impl QIcon {
    pub fn new() -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QIcon_new();
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(Self { this })
        }
    }

    pub fn add_pixmap(&mut self, pixmap: &QPixmap) {
        unsafe {
            crate::platform::qt::qt_wrapper::QIcon_addPixmap(self.this, pixmap.this, 0, 1);
        }
    }
}

impl Drop for QIcon {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QIcon_delete(self.this);
        }
    }
}
