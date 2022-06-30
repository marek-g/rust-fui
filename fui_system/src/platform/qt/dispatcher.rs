use crate::platform::qt::qt_wrapper::QApplication;
use std::marker::PhantomData;

///
/// Allows to communicate with a message loop from the same thread.
///
pub struct Dispatcher {
    // impl !Send for LoopProxy {}
    _marker: PhantomData<*const ()>,
}

impl Dispatcher {
    pub(crate) fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    ///
    /// Post function to be executed from the message loop.
    ///
    pub fn post_func_any_thread<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        QApplication::post_func_any_thread(func);
    }

    ///
    /// Post function to be executed from the message loop.
    ///
    pub fn post_func_same_thread<F>(&self, func: F)
    where
        F: FnOnce() + 'static,
    {
        unsafe {
            QApplication::post_func_same_thread(func);
        }
    }
}
