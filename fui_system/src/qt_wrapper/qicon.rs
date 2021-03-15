use crate::qt_wrapper::QPixmap;

pub struct QIcon {
    pub this: *mut ::std::os::raw::c_void,
}

impl QIcon {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let this = crate::qt_wrapper::QIcon_new();
            if this.is_null() {
                return Err(());
            }

            Ok(Self { this })
        }
    }

    pub fn add_pixmap(&mut self, pixmap: &QPixmap) -> Result<(), ()> {
        unsafe {
            crate::qt_wrapper::QIcon_addPixmap(self.this, pixmap.this, 0, 1);
            Ok(())
        }
    }
}

impl Drop for QIcon {
    fn drop(&mut self) {
        unsafe {
            crate::qt_wrapper::QIcon_delete(self.this);
        }
    }
}
