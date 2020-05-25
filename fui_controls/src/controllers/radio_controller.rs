use std::cell::RefCell;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

use fui::{ EventSubscription, ObservableCollection };

pub trait RadioElement {
    fn is_checked(&self) -> bool;
    fn set_is_checked(&mut self, is_checked: bool);
    fn on_checked(&self, f: Box<dyn Fn()>) -> EventSubscription;
}

pub struct RadioController {
    elements: Rc<RefCell<dyn ObservableCollection<Rc<RefCell<dyn RadioElement>>>>>,
    subscriptions: Rc<RefCell<Vec<EventSubscription>>>,
}

impl RadioController {
    pub fn new(elements: Rc<RefCell<dyn ObservableCollection<Rc<RefCell<dyn RadioElement>>>>>) -> Self {
        let mut subscriptions = Vec::new();

        for radio_element in elements.borrow_mut().deref() {
            let elements_clone = elements.clone();
            let radio_element_clone = radio_element.clone();
            subscriptions.push(
                radio_element.borrow().on_checked(Box::new(
                    move || {
                        for el in elements_clone.borrow_mut().deref() {
                            if !Rc::ptr_eq(&el, &radio_element_clone) {
                                el.borrow_mut().set_is_checked(false);
                            }
                        }
                    }
                ))
            );
        }

        let is_checked = elements.borrow_mut().into_iter().fold(false,
            |acc, el| acc || el.borrow().is_checked());
        if !is_checked {
            if let Some(el) = elements.borrow_mut().into_iter().next() {
                el.borrow_mut().set_is_checked(true);
            }
        }

        let subscriptions = Rc::new(RefCell::new(subscriptions));
        let subscriptions_clone = subscriptions.clone();

        let elements_clone = elements.clone();
        elements.borrow_mut().on_changed(Box::new(
            move |args| match args {
                fui::ObservableChangedEventArgs::Insert { index, value: radio_element } => {
                    if elements_clone.borrow().len() == 0 {
                        radio_element.borrow_mut().set_is_checked(true);
                    }

                    let elements_clone = elements_clone.clone();
                    let radio_element_clone = radio_element.clone();
                    let subscription = radio_element.borrow().on_checked(Box::new(
                        move || {
                            for el in elements_clone.borrow_mut().deref() {
                                if !Rc::ptr_eq(&el, &radio_element_clone) {
                                    el.borrow_mut().set_is_checked(false);
                                }
                            }
                        }
                    ));
                    subscriptions_clone.borrow_mut().insert(index, subscription);
                }
                fui::ObservableChangedEventArgs::Remove { index } => {
                    subscriptions_clone.borrow_mut().remove(index);
                }
            }
        ));

        RadioController {
            elements,
            subscriptions,
        }
    }
}
