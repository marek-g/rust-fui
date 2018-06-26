use std::rc::Rc;
use std::cell::RefCell;

use Binding;
use BindingData;
use Event;
use EventSubscription;

pub struct Property<T> {
    data: Rc<PropertyData<T>>
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

    pub fn bind(&mut self, src_property: &mut Property<T>) -> Box<Binding> {
        self.set(src_property.get());

        let weak_data = Rc::downgrade(&self.data);
        let event_subscription = src_property.data.changed.borrow_mut().subscribe(move |src_val| {
            if let Some(dest_property_data) = weak_data.upgrade() {
                dest_property_data.set(src_val.clone());
            }
        });
        Box::new(BindingData { subscription: event_subscription })
    }

    pub fn bind_c<TSrc: 'static + Clone + PartialEq, F: 'static + Fn(TSrc) -> T>(&mut self,
        src_property: &mut Property<TSrc>, f: F) -> Box<Binding> {
        self.set(f(src_property.get()));

        let weak_data = Rc::downgrade(&self.data);
        let boxed_f = Box::new(f);
        let event_subscription = src_property.data.changed.borrow_mut().subscribe(move |src_val| {
            if let Some(dest_property_data) = weak_data.upgrade() {
                dest_property_data.set(boxed_f(src_val));
            }
        });
        Box::new(BindingData { subscription: event_subscription })
    }

    pub fn on_changed<F: 'static + Fn(T)>(&mut self, f: F) -> EventSubscription<T> {
        self.data.changed.borrow_mut().subscribe(f)
    }

    pub fn on_changed_without_subscription<F: 'static + Fn(T)>(&mut self, f: F) {
        self.data.changed.borrow_mut().subscribe_without_subscription(f);
    }
}

pub struct PropertyData<T> {
    value: RefCell<T>,
    changed: RefCell<Event<T>>
}

impl<T: 'static + Clone + PartialEq> PropertyData<T> {
    pub fn new(val: T) -> Self {
        PropertyData {
            value: RefCell::new(val),
            changed: RefCell::new(Event::new())
        }
    }

    pub fn set(&self, val: T) {
        let old_value = self.value.replace(val.clone());
        if old_value != val {
            self.changed.borrow().emit(val);
        }
    }

    pub fn get(&self) -> T {
        (*self.value.borrow()).clone()
    }
}
