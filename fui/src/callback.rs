pub struct Callback<'a, A> {
    callback: Option<Box<'a + FnMut(A)>>
}

impl<'a, A> Callback<'a, A> {
    pub fn new() -> Self {
        Callback { callback: None }
    }

    pub fn set<F: 'a + FnMut(A)>(&mut self, f: F) {
        self.callback = Some(Box::new(f));
    }

    pub fn emit(&mut self, args: A) {
        if let Some(ref mut f) = self.callback {
            f(args);
        }
    }
}
