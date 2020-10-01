use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use crate::Callback;

///
/// EventSubscription is an owner of the callback (handler).
/// Event keeps only a weak reference to the callback (handler).
///
/// Calling the callback stops when EventSubscription is dropped.
///
/// EventSubscription can keep more than one callback internally.
/// It is useful when implementing aggregation of events.
///
pub struct EventSubscription {
    _callbacks: Vec<Rc<dyn CallbackObject>>,
}

impl EventSubscription {
    pub fn from_many(event_subscriptions: Vec<EventSubscription>) -> Self {
        let mut callbacks = Vec::new();
        for mut subscription in event_subscriptions.into_iter() {
            callbacks.append(&mut subscription._callbacks);
        }
        EventSubscription {
            _callbacks: callbacks,
        }
    }
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

    pub fn subscribe<F: 'static + FnMut(A)>(&mut self, f: F) -> EventSubscription {
        let mut callback = Callback::<A>::empty();
        callback.set(f);
        let rc_callback = Rc::new(callback);
        let weak_callback = Rc::downgrade(&rc_callback);
        self.callbacks.borrow_mut().push(weak_callback);

        EventSubscription {
            _callbacks: vec![rc_callback],
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
