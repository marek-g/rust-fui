use crate::FUISystemError;

pub struct QByteArray {
    pub this: *mut ::std::os::raw::c_void,
}

impl QByteArray {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QByteArray_new();
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(Self { this })
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = crate::platform::qt::qt_wrapper::QByteArray_constData(self.this);
            let length = crate::platform::qt::qt_wrapper::QByteArray_size(self.this);
            std::slice::from_raw_parts(ptr as *const u8, length as usize)
        }
    }
}

impl Drop for QByteArray {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QByteArray_delete(self.this);
        }
    }
}
