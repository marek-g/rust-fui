use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

///
/// EventSubscription is an owner of the callback.
/// Calling the callback stops when EventSubscription is dropped.
///
pub struct EventSubscription<A> {
    _callback: Rc<Box<RefCell<'static + FnMut(&A)>>>
}

pub struct Event<A> {
    pub callbacks: Vec<Weak<Box<RefCell<'static + FnMut(&A)>>>>
}

impl<A> Event<A> {
    pub fn new() -> Self {
        Event {
            callbacks: Vec::new()
        }
    }

    pub fn subscribe<F: 'static + FnMut(&A)>(&mut self, f: F) -> EventSubscription<A> {
        let box_callback: Box<RefCell<FnMut(&A)>> = Box::new(RefCell::new(f));
        let rc_callback = Rc::new(box_callback);
        let weak_callback = Rc::downgrade(&rc_callback);

        self.callbacks.push(weak_callback);
        EventSubscription { _callback: rc_callback }
    }

    pub fn emit(&mut self, args: &A) {
        let mut cleanup = false;

        for weak_callback in self.callbacks.iter() {
            if let Some(mut ref_cell_f) = weak_callback.upgrade() {
                let f = &mut *ref_cell_f.borrow_mut();
                f(&args)
            } else {
                cleanup = true;
            }
        }

        if cleanup {
            self.callbacks.retain(|ref weak_callback| {
                let got_ref = weak_callback.clone().upgrade();
                match got_ref { None => false, _ => true }
            });
        }
    }
}
