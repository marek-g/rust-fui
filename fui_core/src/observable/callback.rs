use crate::spawn_local_and_forget;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

///
/// Callback can hold one listener that can be queued to execute with emit() method.
/// The real execution will be done later on the same thread.
/// Callbacks are queued because this prevents mutability problems when callbacks are called from callbacks
/// and you are using Rc<T>.
///
/// Callback is the owner of the listener closure.
///
/// Before the first use, you must register your Dispatcher abstraction
/// with register_current_thread_dispatcher() function.
///
#[derive(Clone)]
pub struct Callback<A> {
    callback: Rc<RefCell<Option<Box<dyn 'static + FnMut(A)>>>>,
}

impl<A: 'static + Clone> Callback<A> {
    pub fn empty() -> Self {
        Callback {
            callback: Rc::new(RefCell::new(None)),
        }
    }

    pub fn new_sync<F: 'static + FnMut(A)>(f: F) -> Self {
        let mut callback = Callback::empty();
        callback.set_sync(f);
        callback
    }

    pub fn new_async<F, Fut>(f: F) -> Self
    where
        F: FnMut(A) -> Fut + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        let mut callback = Callback::empty();
        callback.set_async(f);
        callback
    }

    pub fn new_vm<T: 'static, F: 'static + FnMut(&T, A)>(vm: &Rc<T>, f: F) -> Self {
        let mut callback = Callback::empty();
        callback.set_vm(vm, f);
        callback
    }

    pub fn new_vm_rc<T: 'static, F: 'static + FnMut(Rc<T>, A)>(vm: &Rc<T>, f: F) -> Self {
        let mut callback = Callback::empty();
        callback.set_vm_rc(vm, f);
        callback
    }

    pub fn set_sync<F: 'static + FnMut(A)>(&mut self, f: F) {
        *self.callback.borrow_mut() = Some(Box::new(f));
    }

    pub fn set_async<F, Fut>(&mut self, mut f: F)
    where
        F: FnMut(A) -> Fut + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        let f2 = move |args: A| {
            spawn_local_and_forget(f(args));
        };

        *self.callback.borrow_mut() = Some(Box::new(f2));
    }

    pub fn set_vm<T: 'static, F: 'static + FnMut(&T, A)>(&mut self, vm: &Rc<T>, mut f: F) {
        let vm_clone = vm.clone();
        let f2 = move |args: A| {
            f(&vm_clone, args);
        };

        *self.callback.borrow_mut() = Some(Box::new(f2));
    }

    pub fn set_vm_rc<T: 'static, F: 'static + FnMut(Rc<T>, A)>(&mut self, vm: &Rc<T>, mut f: F) {
        let vm_clone = vm.clone();
        let f2 = move |args: A| {
            let vm = vm_clone.clone();
            f(vm, args);
        };

        *self.callback.borrow_mut() = Some(Box::new(f2));
    }

    pub fn clear(&mut self) {
        *self.callback.borrow_mut() = None;
    }

    pub fn emit(&self, args: A) {
        if self.callback.borrow_mut().is_some() {
            let weak = Rc::downgrade(&self.callback);
            spawn_local_and_forget(async move {
                if let Some(f) = weak.upgrade() {
                    if let Some(f2) = &mut *f.borrow_mut() {
                        f2(args);
                    }
                }
            });
        }
    }
}
