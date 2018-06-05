use std::cell::RefCell;
use std::rc::Rc;

///
/// Callback can hold one listener that can be called any time with emit() method.
///
/// Callback is the owner of the listener clousure.
///
pub struct Callback<A> {
    callback: Option<Box<'static + Fn(&A)>>
}

impl<A> Callback<A> {
    pub fn new() -> Self {
        Callback { callback: None }
    }

    pub fn set<F: 'static + Fn(&A)>(&mut self, f: F) {
        self.callback = Some(Box::new(f));
    }

    pub fn set_vm<T: 'static, F: 'static + Fn(&mut T, &A)>(&mut self, vm: &Rc<RefCell<T>>, f: F) {
        let vm_clone = vm.clone();
        let f2 = move |args: &A| {
            let mut vm = vm_clone.borrow_mut();
            f(&mut vm, args);
        };
        self.callback = Some(Box::new(f2));
    }

    pub fn clear(&mut self) {
        self.callback = None;
    }

    pub fn emit(&self, args: &A) {
        if let Some(ref f) = self.callback {
            f(args)
        }
    }
}
