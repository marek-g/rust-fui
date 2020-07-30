use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use crate::Callback;

///
/// EventSubscription is an owner of the callback.
/// Calling the callback stops when EventSubscription is dropped.
///
pub struct EventSubscription {
    _callback: Rc<dyn CallbackObject>,
}

pub trait CallbackObject {}
impl<A> CallbackObject for Callback<A> {}

pub struct Event<A> {
    callbacks: RefCell<Vec<Weak<Callback<A>>>>,
}

impl<A: 'static + Clone> Event<A> {
    pub fn new() -> Self {
        Event {
            callbacks: RefCell::new(Vec::new()),
        }
    }

    pub fn subscribe<F: 'static + Fn(A)>(&mut self, f: F) -> EventSubscription {
        let mut callback = Callback::<A>::empty();
        callback.set(f);
        let rc_callback = Rc::new(callback);
        let weak_callback = Rc::downgrade(&rc_callback);
        self.callbacks.borrow_mut().push(weak_callback);

        EventSubscription {
            _callback: rc_callback,
        }
    }

    pub fn emit(&self, args: A) {
        let mut cleanup = false;

        for weak_callback in self.callbacks.borrow().iter() {
            if let Some(callback) = weak_callback.upgrade() {
                callback.emit(args.clone());
            } else {
                cleanup = true;
            }
        }

        if cleanup {
            self.callbacks.borrow_mut().retain(|ref weak_callback| {
                let got_ref = weak_callback.clone().upgrade();
                match got_ref {
                    None => false,
                    _ => true,
                }
            });
        }
    }
}
