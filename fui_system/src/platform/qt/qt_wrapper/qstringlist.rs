use crate::platform::qt::qt_wrapper::QString;
use crate::FUISystemError;

pub struct QStringList {
    pub this: *mut ::std::os::raw::c_void,
}

impl QStringList {
    pub fn get(&self, pos: usize) -> Result<String, FUISystemError> {
        unsafe {
            let ptr = crate::platform::qt::qt_wrapper::QStringList_at(self.this, pos as i32);
            let qstring = QString {
                this: ptr as *mut std::ffi::c_void,
                is_owned: false,
            };
            qstring.as_string()
        }
    }

    pub fn size(&self) -> usize {
        unsafe { crate::platform::qt::qt_wrapper::QStringList_size(self.this) as usize }
    }
}

impl Drop for QStringList {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QStringList_delete(self.this);
        }
    }
}
