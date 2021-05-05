// https://www.mdeditor.tw/pl/pfFY/zh-hk

use std::ffi::c_void;

// Wrapper for callback.
// Allows to pass the callback to C code.
//
// TODO: Verify if this is safe to use.
// For example, when we drop the MenuItem (owner of RawCallback)
// that was used to initialize menu for tray icon,
// isn't the callback dropped before use?
pub struct RawCallback {
    trampoline_func: unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
    trampoline_func_data: *mut c_void,
    drop_trampoline_func: unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
}

impl RawCallback {
    pub fn new<F>(callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        RawCallback {
            trampoline_func: callback_trampoline::<F>,
            trampoline_func_data: callback_to_pointer(callback),
            drop_trampoline_func: drop_callback_pointer::<F>,
        }
    }

    pub fn get_trampoline_func(&self) -> unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) {
        self.trampoline_func
    }

    pub fn get_trampoline_func_data(&self) -> *mut c_void {
        self.trampoline_func_data
    }

    pub fn get_drop_trampoline_func(
        &self,
    ) -> unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) {
        self.drop_trampoline_func
    }
}

impl Drop for RawCallback {
    fn drop(&mut self) {
        unsafe {
            (self.drop_trampoline_func)(self.trampoline_func_data);
        }
    }
}

pub fn callback_to_pointer<F>(callback: F) -> *mut c_void
where
    F: FnMut() + 'static,
{
    let b = Box::new(callback);
    Box::into_raw(b) as *mut c_void
}

/// This is a function that can be passed to C code
/// along with a pointer to the callback.
///
/// Calling this method from C will call Rust's callback.
pub extern "C" fn callback_trampoline<F>(callback_pointer: *mut c_void)
where
    F: FnMut() + 'static,
{
    let callback_ptr = callback_pointer as *mut F;
    let callback = unsafe { &mut *callback_ptr };
    callback();
}

pub extern "C" fn drop_callback_pointer<T>(callback_pointer: *mut c_void) {
    unsafe {
        Box::from_raw(callback_pointer as *mut T);
    }
}
