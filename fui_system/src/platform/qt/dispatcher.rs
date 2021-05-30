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
    pub fn post_func<F>(&self, func: F)
    where
        F: FnOnce() + 'static,
    {
        QApplication::post_func(func);
    }
}
