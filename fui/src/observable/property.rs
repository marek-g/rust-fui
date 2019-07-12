use std::rc::Rc;
use std::cell::RefCell;

use Event;
use EventSubscription;

pub struct Property<T> {
    data: Rc<PropertyData<T>>,
    binding_subscription: Option<EventSubscription>,
}

impl<T: 'static + Clone + PartialEq> Property<T> {
    pub fn new(val: T) -> Self {
        Property {
            data: Rc::new(PropertyData::new(val)),
            binding_subscription: None,
        }
    }

    pub fn set(&mut self, val: T) {
        self.data.set(val);
    }

    pub fn change<F: 'static + Fn(T) -> T>(&mut self, f: F) {
        let val = self.data.get();
        self.data.set(f(val));
    }

    pub fn get(&self) -> T {
        self.data.get()
    }

    pub fn bind(&mut self, src_property: &mut Property<T>) {
        self.set(src_property.get());

        let weak_data = Rc::downgrade(&self.data);
        self.binding_subscription = Some(src_property.data.changed.borrow_mut().subscribe(move |src_val| {
            if let Some(dest_property_data) = weak_data.upgrade() {
                dest_property_data.set(src_val.clone());
            }
        }))
    }

    pub fn bind_c<TSrc: 'static + Clone + PartialEq, F: 'static + Fn(TSrc) -> T>(&mut self,
        src_property: &mut Property<TSrc>, f: F) {
        self.set(f(src_property.get()));

        let weak_data = Rc::downgrade(&self.data);
        let boxed_f = Box::new(f);
        self.binding_subscription = Some(src_property.data.changed.borrow_mut().subscribe(move |src_val| {
            if let Some(dest_property_data) = weak_data.upgrade() {
                dest_property_data.set(boxed_f(src_val));
            }
        }))
    }

    pub fn on_changed<F: 'static + Fn(T)>(&mut self, f: F) -> EventSubscription {
        self.data.changed.borrow_mut().subscribe(f)
    }
}

struct PropertyData<T> {
    value: RefCell<T>,
    changed: RefCell<Event<T>>
}

impl<T: 'static + Clone + PartialEq> PropertyData<T> {
    fn new(val: T) -> Self {
        PropertyData {
            value: RefCell::new(val),
            changed: RefCell::new(Event::new())
        }
    }

    fn set(&self, val: T) {
        let old_value = self.value.replace(val.clone());
        if old_value != val {
            self.changed.borrow().emit(val);
        }
    }

    fn get(&self) -> T {
        (*self.value.borrow()).clone()
    }
}

impl<T> From<T> for Property<T> where T: 'static + Clone + PartialEq {
    fn from(value: T) -> Property<T> {
        Property::new(value)
    }
}

impl From<&str> for Property<String> {
    fn from(value: &str) -> Property<String> {
        Property::new(value.to_string())
    }
}
