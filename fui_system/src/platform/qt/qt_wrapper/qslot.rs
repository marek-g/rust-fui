use crate::common::callback_helper::RawCallback;
use std::ffi::c_void;

pub struct QSlot {
    pub this: *mut ::std::os::raw::c_void,
}

impl QSlot {
    pub fn new(raw_callback: &RawCallback) -> Result<Self, ()> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QSlot_new();
            if this.is_null() {
                return Err(());
            }

            let result = Self { this };

            crate::platform::qt::qt_wrapper::QSlot_setFunc(
                result.this,
                Some(raw_callback.get_trampoline_func()),
                raw_callback.get_trampoline_func_data(),
            );

            Ok(result)
        }
    }
}

impl Drop for QSlot {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QSlot_delete(self.this);
        }
    }
}
