pub struct QMenu {
    pub this: *mut ::std::os::raw::c_void,
}

impl QMenu {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let this = crate::qt_wrapper::QMenu_new();
            if this.is_null() {
                return Err(());
            }

            Ok(Self { this })
        }
    }
}

impl Drop for QMenu {
    fn drop(&mut self) {
        unsafe {
            crate::qt_wrapper::QMenu_delete(self.this);
        }
    }
}
