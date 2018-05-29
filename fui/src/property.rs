use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

use Event;
use EventSubscription;

pub struct Property<T> {
    pub data: Rc<RefCell<PropertyData<T>>>
}

impl<T: 'static + Copy> Property<T> {
    pub fn new(val: T) -> Self {
        Property {
            data: Rc::new(RefCell::new(PropertyData::new(val)))
        }
    }

    pub fn set(&self, val: T) {
        self.data.borrow_mut().set(val);
    }

    pub fn get(&self) -> T {
        self.data.borrow().value
    }

    pub fn bind<TSrc, F: 'static + FnMut(&TSrc) -> T>(&mut self, src_property: &mut Property<TSrc>, f: F) -> EventSubscription<TSrc> {
        let weak_data = Rc::downgrade(&self.data);
        let boxed_f = Box::new(RefCell::new(f));
        src_property.data.borrow_mut().changed.subscribe(move |src_val| {
            if let Some(ref_cell_dest_property_data) = weak_data.upgrade() {
                let dest_property_data = &mut *ref_cell_dest_property_data.borrow_mut();
                let f = &mut *boxed_f.borrow_mut();
                dest_property_data.set(f(src_val));
            }
        })
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
