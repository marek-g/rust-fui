pub struct Callback<A> {
    callback: Option<Box<'static + FnMut(A)>>
}

impl<A> Callback<A> {
    pub fn new() -> Self {
        Callback { callback: None }
    }

    pub fn set<F: 'static + FnMut(A)>(&mut self, f: F) {
        self.callback = Some(Box::new(f));
    }

    pub fn clear(&mut self) {
        self.callback = None;
    }

    pub fn emit(&mut self, args: A) {
        if let Some(ref mut f) = self.callback {
            f(args)
        }
    }
}
