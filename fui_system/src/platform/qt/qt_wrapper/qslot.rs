use crate::common::callback_helper::RawCallback;
use crate::FUISystemError;

pub struct QSlot {
    pub this: *mut ::std::os::raw::c_void,
    raw_callback: RawCallback,
}

impl QSlot {
    pub fn new(raw_callback: RawCallback) -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QSlot_new();
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            let result = Self { this, raw_callback };

            crate::platform::qt::qt_wrapper::QSlot_setFunc(
                result.this,
                Some(result.raw_callback.get_trampoline_func()),
                result.raw_callback.get_trampoline_func_data(),
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
