pub struct Event {
    pub callback: Box<FnMut()>
}

impl Event {
    pub fn new<F: 'static + FnMut()>(f: F) -> Self {
        Event {
            callback: Box::new(f)
        }
    }

    pub fn set<F: 'static + FnMut()>(&mut self, f: F) {
        self.callback = Box::new(f);
    }

    pub fn emit(&mut self) {
        (self.callback)();
    }
}
