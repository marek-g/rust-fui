use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use Callback;

///
/// EventSubscription is an owner of the callback.
/// Calling the callback stops when EventSubscription is dropped.
///
pub struct EventSubscription<A> {
    _callback: Rc<Callback<A>>
}

pub struct Event<A> {
    callbacks: RefCell<Vec<Weak<Callback<A>>>>,
    subscriptions: Vec<EventSubscription<A>>,
}

impl<A> Event<A> {
    pub fn new() -> Self {
        Event {
            callbacks: RefCell::new(Vec::new()),
            subscriptions: Vec::new(),
        }
    }

    pub fn subscribe<F: 'static + Fn(&A)>(&mut self, f: F) -> EventSubscription<A> {
        let mut callback = Callback::<A>::new();
        callback.set(f);
        let rc_callback = Rc::new(callback);
        let weak_callback = Rc::downgrade(&rc_callback);

        self.callbacks.borrow_mut().push(weak_callback);
        EventSubscription { _callback: rc_callback }
    }

    pub fn subscribe_without_subscription<F: 'static + Fn(&A)>(&mut self, f: F) {
        let subscription = self.subscribe(f);
        self.subscriptions.push(subscription);
    }

    pub fn emit(&self, args: &A) {
        let mut cleanup = false;

        for weak_callback in self.callbacks.borrow().iter() {
            if let Some(callback) = weak_callback.upgrade() {
                callback.emit(args);
            } else {
                cleanup = true;
            }
        }

        if cleanup {
            self.callbacks.borrow_mut().retain(|ref weak_callback| {
                let got_ref = weak_callback.clone().upgrade();
                match got_ref { None => false, _ => true }
            });
        }
    }
}
