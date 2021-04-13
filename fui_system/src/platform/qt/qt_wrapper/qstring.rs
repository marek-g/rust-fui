pub struct QString {
    pub this: *mut ::std::os::raw::c_void,
}

impl QString {
    pub fn from_str(text: &str) -> Result<Self, ()> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QString_fromUtf8(
                text.as_ptr() as *const i8,
                text.len() as i32,
            );
            if this.is_null() {
                return Err(());
            }

            Ok(Self { this })
        }
    }
}

impl Drop for QString {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QString_delete(self.this);
        }
    }
}
