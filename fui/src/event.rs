use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

pub struct Event<A> {
    pub callback: Option<Weak<Box<RefCell<FnMut(&A)>>>>
}

impl<A> Event<A> {
    pub fn new() -> Self {
        Event {
            callback: None
        }
    }

    pub fn subscribe<F: 'static + FnMut(&A)>(&mut self, f: F) -> EventSubscription<A> {
        let box_callback: Box<RefCell<FnMut(&A)>> = Box::new(RefCell::new(f));
        let rc_callback = Rc::new(box_callback);
        self.callback = Some(Rc::downgrade(&rc_callback));
        EventSubscription { _callback: rc_callback }
    }

    pub fn emit(&mut self, args: &A) {
        let mut cleanup = false;

        if let Some(ref weak_f) = &self.callback {
            if let Some(mut ref_cell_f) = weak_f.upgrade() {
                let f = &mut *ref_cell_f.borrow_mut();
                f(&args)
            } else {
                cleanup = true;
            }
        }

        if cleanup {
            self.callback = None
        }
    }
}

pub struct EventSubscription<A> {
    _callback: Rc<Box<RefCell<FnMut(&A)>>>
}

impl<A> Drop for EventSubscription<A> {
    fn drop(&mut self) {
        println!("Drop!")
    }
}