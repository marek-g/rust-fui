use std::ffi::c_void;

pub struct QSlot {
    pub this: *mut ::std::os::raw::c_void,
}

extern "C" fn callback(target: *mut c_void) {
    println!("I'm called from C");
    unsafe {
        // Update the value in RustObject with the value received from the callback:
        //(*target).a = a;
    }
}

impl QSlot {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QSlot_new();
            if this.is_null() {
                return Err(());
            }

            let result = Self { this };

            println!("Setfunc!");
            crate::platform::qt::qt_wrapper::QSlot_setFunc(
                result.this,
                Some(callback),
                0 as *mut c_void,
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
