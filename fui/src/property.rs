use std::rc::Rc;
use std::cell::RefCell;

use Binding;
use BindingData;
use Event;

pub struct Property<T> {
    pub data: Rc<PropertyData<T>>
}

impl<T: 'static + Clone + PartialEq> Property<T> {
    pub fn new(val: T) -> Self {
        Property {
            data: Rc::new(PropertyData::new(val))
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

    pub fn bind<TSrc: 'static + Clone + PartialEq, F: 'static + Fn(&TSrc) -> T>(&mut self,
        src_property: &mut Property<TSrc>, f: F) -> Box<Binding> {
        self.set(f(&src_property.get()));

        let weak_data = Rc::downgrade(&self.data);
        let boxed_f = Box::new(f);
        let event_subscription = src_property.data.changed.subscribe(move |src_val| {
            if let Some(dest_property_data) = weak_data.upgrade() {
                dest_property_data.set(boxed_f(src_val));
            }
        });
        Box::new(BindingData { subscription: event_subscription })
    }
}

pub struct PropertyData<T> {
    pub value: RefCell<T>,
    pub changed: Event<T>
}

impl<T: 'static + Clone + PartialEq> PropertyData<T> {
    pub fn new(val: T) -> Self {
        PropertyData {
            value: RefCell::new(val),
            changed: Event::new()
        }
    }

    pub fn set(&self, val: T) {
        let old_value = self.value.replace(val.clone());
        if old_value != val {
            self.changed.emit(&val);
        }
    }

    pub fn get(&self) -> T {
        (*self.value.borrow()).clone()
    }
}
