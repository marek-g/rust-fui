use Event;

pub struct Property<T> {
    value: T,
    changed: Event<T>
}

impl<T> Property<T> {
    pub fn new(val: T) -> Self {
        Property {
            value: val,
            changed: Event::new()
        }
    }

    pub fn set(&mut self, val: T) {
        self.value = val;
        self.changed.emit(&self.value);
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    /*pub fn bind<TSrc, F: 'static + FnMut(&TSrc) -> T>(&mut self, src_property: &mut Property<TSrc>, f: F) {
        src_property.changed.subscribe(|src_val| { self.set(f(src_val)); });
    }*/
}
