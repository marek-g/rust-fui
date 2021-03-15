pub struct QPixmap {
    pub this: *mut ::std::os::raw::c_void,
}

impl QPixmap {
    pub fn from_data(data: &[u8]) -> Result<Self, ()> {
        unsafe {
            let this = crate::qt_wrapper::QPixmap_new();
            if this.is_null() {
                return Err(());
            }

            if crate::qt_wrapper::QPixmap_loadFromData(this, data.as_ptr(), data.len() as i32) == 0
            {
                return Err(());
            }

            Ok(Self { this })
        }
    }
}

impl Drop for QPixmap {
    fn drop(&mut self) {
        unsafe {
            crate::qt_wrapper::QPixmap_delete(self.this);
        }
    }
}
