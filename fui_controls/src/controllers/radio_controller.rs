use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::ToggleButton;
use fui_core::{EventSubscription, ObservableCollection, StyledControl};

pub trait RadioElement {
    fn is_checked(&self) -> bool;
    fn set_is_checked(&mut self, is_checked: bool);
    fn on_checked(&self, f: Box<dyn Fn()>) -> EventSubscription;
}

impl RadioElement for StyledControl<ToggleButton> {
    fn is_checked(&self) -> bool {
        self.data.is_checked.get()
    }

    fn set_is_checked(&mut self, is_checked: bool) {
        self.data.is_checked.set(is_checked)
    }

    fn on_checked(&self, f: Box<dyn Fn()>) -> EventSubscription {
        self.data.is_checked.on_changed(move |is_checked| {
            if is_checked {
                f();
            }
        })
    }
}

pub struct RadioController<R>
where
    R: 'static + RadioElement,
{
    elements: Rc<RefCell<dyn ObservableCollection<Rc<RefCell<R>>>>>,
    subscriptions: Rc<RefCell<Vec<EventSubscription>>>,
}

impl<R> RadioController<R>
where
    R: 'static + RadioElement,
{
    pub fn new<E>(elements: E) -> Self
    where
        E: 'static + ObservableCollection<Rc<RefCell<R>>>,
    {
        let mut subscriptions = Vec::new();
        let elements: Rc<RefCell<dyn ObservableCollection<Rc<RefCell<R>>>>> =
            Rc::new(RefCell::new(elements));

        for radio_element in elements.borrow_mut().deref() {
            let elements_clone = elements.clone();
            let radio_element_clone = radio_element.clone();
            subscriptions.push(radio_element.borrow().on_checked(Box::new(move || {
                for el in elements_clone.borrow_mut().deref() {
                    if !Rc::ptr_eq(&el, &radio_element_clone) {
                        el.borrow_mut().set_is_checked(false);
                    }
                }
            })));
        }

        let is_checked = elements
            .borrow_mut()
            .into_iter()
            .fold(false, |acc, el| acc || el.borrow().is_checked());
        if !is_checked {
            if let Some(el) = elements.borrow_mut().into_iter().next() {
                el.borrow_mut().set_is_checked(true);
            }
        }

        let subscriptions = Rc::new(RefCell::new(subscriptions));
        let subscriptions_clone = subscriptions.clone();

        let elements_clone = elements.clone();
        elements
            .borrow_mut()
            .on_changed(Box::new(move |args| match args {
                fui_core::ObservableChangedEventArgs::Insert {
                    index,
                    value: radio_element,
                } => {
                    if elements_clone.borrow().len() == 0 {
                        radio_element.borrow_mut().set_is_checked(true);
                    }

                    let elements_clone = elements_clone.clone();
                    let radio_element_clone = radio_element.clone();
                    let subscription = radio_element.borrow().on_checked(Box::new(move || {
                        for el in elements_clone.borrow_mut().deref() {
                            if !Rc::ptr_eq(&el, &radio_element_clone) {
                                el.borrow_mut().set_is_checked(false);
                            }
                        }
                    }));
                    subscriptions_clone.borrow_mut().insert(index, subscription);
                }
                fui_core::ObservableChangedEventArgs::Remove { index } => {
                    subscriptions_clone.borrow_mut().remove(index);
                }
            }));

        RadioController {
            elements,
            subscriptions,
        }
    }
}
