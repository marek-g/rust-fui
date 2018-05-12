pub struct Event<A> {
    pub callback: Option<Box<FnMut(A)>>
}

impl<A> Event<A> {
    pub fn new() -> Self {
        Event {
            callback: None
        }
    }

    pub fn set<F: 'static + FnMut(A)>(&mut self, f: F) {
        self.callback = Some(Box::new(f));
    }

    pub fn emit(&mut self, args: A) {
        if let Some(ref mut f) = &mut self.callback {
            f(args);
        }
    }
}
