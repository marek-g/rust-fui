use crate::platform::qt::qt_wrapper::QApplication;
use std::marker::PhantomData;

/// Allows to communicate with message loop from the same thread.
pub struct LoopProxy {
    // impl !Send for LoopProxy {}
    _marker: PhantomData<*const ()>,
}

impl LoopProxy {
    pub(crate) fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn post_func<F>(&self, func: F)
    where
        F: FnOnce() + 'static,
    {
        QApplication::post_func(func);
    }
}
