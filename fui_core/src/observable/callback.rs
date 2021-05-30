use crate::post_func_current_thread;
use std::cell::RefCell;
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
    callback: Option<Rc<RefCell<dyn 'static + FnMut(A)>>>,
}

impl<A: 'static + Clone> Callback<A> {
    pub fn empty() -> Self {
        Callback { callback: None }
    }

    pub fn new<T: 'static, F: 'static + FnMut(&mut T, A)>(vm: &Rc<RefCell<T>>, mut f: F) -> Self {
        let vm_clone = vm.clone();
        let f2 = move |args: A| {
            let mut vm = vm_clone.borrow_mut();
            f(&mut vm, args);
        };
        Callback {
            callback: Some(Rc::new(RefCell::new(f2))),
        }
    }

    pub fn new_rc<T: 'static, F: 'static + FnMut(Rc<RefCell<T>>, A)>(
        vm: &Rc<RefCell<T>>,
        mut f: F,
    ) -> Self {
        let vm_clone = vm.clone();
        let f2 = move |args: A| {
            let vm = vm_clone.clone();
            f(vm, args);
        };
        Callback {
            callback: Some(Rc::new(RefCell::new(f2))),
        }
    }

    pub fn simple<F: 'static + FnMut(A)>(f: F) -> Self {
        Self {
            callback: Some(Rc::new(RefCell::new(f))),
        }
    }

    pub fn set<F: 'static + FnMut(A)>(&mut self, f: F) {
        self.callback = Some(Rc::new(RefCell::new(f)));
    }

    pub fn set_vm<T: 'static, F: 'static + FnMut(&mut T, A)>(
        &mut self,
        vm: &Rc<RefCell<T>>,
        mut f: F,
    ) {
        let vm_clone = vm.clone();
        let f2 = move |args: A| {
            let mut vm = vm_clone.borrow_mut();
            f(&mut vm, args);
        };
        self.callback = Some(Rc::new(RefCell::new(f2)));
    }

    pub fn clear(&mut self) {
        self.callback = None;
    }

    pub fn emit(&self, args: A) {
        if let Some(f) = &self.callback {
            let weak = Rc::downgrade(f);
            post_func_current_thread(Box::new(move || {
                if let Some(f) = weak.upgrade() {
                    f.borrow_mut()(args);
                }
            }));
        }
    }
}
