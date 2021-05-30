use std::cell::RefCell;

thread_local! {
   //static THREAD_DISPATCHER: RefCell<Option<Rc<RefCell<Box<dyn Dispatcher>>>>> = RefCell::new(None);
    static THREAD_DISPATCHER: RefCell<Option<Box<dyn Dispatcher>>> = RefCell::new(None);
}

///
/// Allows to communicate with a message loop from the same thread.
///
pub trait Dispatcher {
    ///
    /// Post function to be executed from the message loop.
    ///
    fn post_func(&mut self, func: Box<dyn FnOnce() + 'static>);
}

pub fn register_current_thread_dispatcher(dispatcher: Box<dyn Dispatcher>) {
    THREAD_DISPATCHER.with(|d| *d.borrow_mut() = Some(dispatcher));
}

pub fn post_func_current_thread(func: Box<dyn FnOnce() + 'static>) {
    THREAD_DISPATCHER.with(|d| d.borrow_mut().as_mut().unwrap().post_func(func))
}
