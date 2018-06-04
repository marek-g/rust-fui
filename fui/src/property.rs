use std::rc::Rc;
use std::cell::RefCell;

use Binding;
use BindingData;
use Event;

pub struct Property<T> {
    pub data: Rc<RefCell<PropertyData<T>>>
}

impl<T: 'static + Clone> Property<T> {
    pub fn new(val: T) -> Self {
        Property {
            data: Rc::new(RefCell::new(PropertyData::new(val)))
        }
    }

    pub fn set(&self, val: T) {
        self.data.borrow_mut().set(val);
    }

    pub fn get(&self) -> T {
        self.data.borrow().value.clone()
    }

    pub fn bind<TSrc: 'static, F: 'static + Fn(&TSrc) -> T>(&self, src_property: &Property<TSrc>, f: F) -> Box<Binding> {
        let weak_data = Rc::downgrade(&self.data);
        let boxed_f = Box::new(f);
        let event_subscription = src_property.data.borrow_mut().changed.subscribe(move |src_val| {
            if let Some(ref_cell_dest_property_data) = weak_data.upgrade() {
                let dest_property_data = &mut *ref_cell_dest_property_data.borrow_mut();
                dest_property_data.set(boxed_f(src_val));
            }
        });
        Box::new(BindingData { subscription: event_subscription })
    }
}

pub struct PropertyData<T> {
    pub value: T,
    pub changed: Event<T>
}

impl<T> PropertyData<T> {
    pub fn new(val: T) -> Self {
        PropertyData {
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
}
