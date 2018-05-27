use std::cell::RefCell;
use std::rc::Rc;

pub struct Callback<A> {
    callback: Option<Rc<RefCell<'static + FnMut(A)>>>
}

impl<A> Callback<A> {
    pub fn new() -> Self {
        Callback { callback: None }
    }

    pub fn set<F: 'static + FnMut(A)>(&mut self, f: F) {
        self.callback = Some(Rc::new(RefCell::new(f)));
    }

    pub fn emit(&mut self, args: A) {
        if let Some(ref ref_cell_f) = self.callback {
            let f = &mut *ref_cell_f.borrow_mut();
            f(args)
        }
    }
}
