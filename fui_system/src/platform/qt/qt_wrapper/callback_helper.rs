// https://www.mdeditor.tw/pl/pfFY/zh-hk

use std::ffi::c_void;
use std::marker::PhantomData;

pub struct RawCallback<F>
where
    F: FnMut() + 'static,
{
    pub ptr: *mut c_void,
    phantom: PhantomData<F>,
}

impl<F> RawCallback<F>
where
    F: FnMut() + 'static,
{
    pub fn new(callback: F) -> Self {
        RawCallback {
            ptr: callback_to_pointer(callback),
            phantom: PhantomData,
        }
    }
}

impl<F> Drop for RawCallback<F>
where
    F: FnMut() + 'static,
{
    fn drop(&mut self) {
        drop_callback_pointer::<F>(self.ptr)
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
