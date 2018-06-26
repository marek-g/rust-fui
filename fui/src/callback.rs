use std::cell::RefCell;
use std::rc::{ Rc, Weak };
use std::collections::VecDeque;

thread_local! {
    static THREAD_CALLBACKS: RefCell<VecDeque<Box<EmittedCallback>>> = RefCell::new(VecDeque::new());
}

///
/// Callback can hold one listener that can be called any time with emit() method.
///
/// Callback is the owner of the listener clousure.
///
pub struct Callback<A> {
    callback: Option<Rc<'static + Fn(A)>>
}

impl<A: 'static + Clone> Callback<A> {
    pub fn new() -> Self {
        Callback { callback: None }
    }

    pub fn set<F: 'static + Fn(A)>(&mut self, f: F) {
        self.callback = Some(Rc::new(f));
    }

    pub fn set_vm<T: 'static, F: 'static + Fn(&mut T, A)>(&mut self, vm: &Rc<RefCell<T>>, f: F) {
        let vm_clone = vm.clone();
        let f2 = move |args: A| {
            let mut vm = vm_clone.borrow_mut();
            f(&mut vm, args);
        };
        self.callback = Some(Rc::new(f2));
    }

    pub fn clear(&mut self) {
        self.callback = None;
    }

    pub fn emit(&self, args: A) {
        if let Some(ref f) = self.callback {
            let e = EmittedCallbackStruct {
                callback: Rc::downgrade(f),
                args: args
            };
            THREAD_CALLBACKS.with(|coll| {
                coll.borrow_mut().push_back(Box::new(e));
            });
            //f(args)
        }
    }
}

pub struct CallbackExecutor;

impl CallbackExecutor {
    pub fn execute_all_in_queue() {
        THREAD_CALLBACKS.with(|coll| {
            while coll.borrow().len() > 0 {
                let emitted = coll.borrow_mut().pop_front();
                if let Some(el) = emitted {
                    el.execute();
                }
            }
        });
    }
}

trait EmittedCallback {
    fn execute(&self);
}

struct EmittedCallbackStruct<A> {
    callback: Weak<'static + Fn(A)>,
    args: A,
}

impl<A: Clone> EmittedCallback for EmittedCallbackStruct<A> {
    fn execute(&self) {
        if let Some(callback) = self.callback.upgrade() {
            callback(self.args.clone());
        }
    }
}
