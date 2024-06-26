use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::ToggleButton;
use fui_core::{ControlObject, ObservableCollection, StyledControl, Subscription, VecDiff};

pub trait RadioElement {
    fn is_checked(&self) -> bool;
    fn set_is_checked(&mut self, is_checked: bool);
    fn on_checked(&self, f: Box<dyn Fn()>) -> Subscription;
}

impl RadioElement for StyledControl<ToggleButton> {
    fn is_checked(&self) -> bool {
        self.data.is_checked.get()
    }

    fn set_is_checked(&mut self, is_checked: bool) {
        self.data.is_checked.set(is_checked)
    }

    fn on_checked(&self, f: Box<dyn Fn()>) -> Subscription {
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
    _elements: Rc<RefCell<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>,
    _subscriptions: Rc<RefCell<Vec<Subscription>>>,

    _phantom_data: PhantomData<R>,
}

impl<R> RadioController<R>
where
    R: 'static + RadioElement,
{
    pub fn new<E>(elements: E) -> Self
    where
        E: 'static + ObservableCollection<Rc<RefCell<dyn ControlObject>>>,
    {
        let mut subscriptions = Vec::new();
        let elements: Rc<RefCell<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>> =
            Rc::new(RefCell::new(elements));

        for radio_element in elements.borrow_mut().deref() {
            subscriptions.push(Self::uncheck_other_when_checked(
                radio_element,
                elements.clone(),
            ));
        }

        let is_checked = elements.borrow_mut().into_iter().fold(false, |acc, el| {
            acc || el
                .borrow()
                .as_any()
                .downcast_ref::<R>()
                .unwrap()
                .is_checked()
        });
        if !is_checked {
            if let Some(el) = elements.borrow_mut().into_iter().next() {
                el.borrow_mut()
                    .as_any_mut()
                    .downcast_mut::<R>()
                    .unwrap()
                    .set_is_checked(true);
            }
        }

        let subscriptions = Rc::new(RefCell::new(subscriptions));
        let subscriptions_clone = subscriptions.clone();

        let elements_clone = elements.clone();
        elements
            .borrow_mut()
            .on_changed(Box::new(move |args| match args {
                VecDiff::Clear {} => {
                    subscriptions_clone.borrow_mut().clear();
                }

                VecDiff::InsertAt {
                    index,
                    value: radio_element,
                } => {
                    if elements_clone.borrow().len() == 0 {
                        radio_element
                            .borrow_mut()
                            .as_any_mut()
                            .downcast_mut::<R>()
                            .unwrap()
                            .set_is_checked(true);
                    }

                    let subscription =
                        Self::uncheck_other_when_checked(radio_element, elements_clone.clone());
                    subscriptions_clone.borrow_mut().insert(index, subscription);
                }

                VecDiff::RemoveAt { index } => {
                    subscriptions_clone.borrow_mut().remove(index);
                }

                VecDiff::Move {
                    old_index,
                    new_index,
                } => {
                    let mut subscriptions = subscriptions_clone.borrow_mut();
                    let subscription = subscriptions.remove(old_index);
                    subscriptions.insert(new_index, subscription);
                }

                VecDiff::Pop {} => {
                    subscriptions_clone.borrow_mut().pop();
                }

                VecDiff::Push {
                    value: radio_element,
                } => {
                    if elements_clone.borrow().len() == 0 {
                        radio_element
                            .borrow_mut()
                            .as_any_mut()
                            .downcast_mut::<R>()
                            .unwrap()
                            .set_is_checked(true);
                    }

                    let subscription =
                        Self::uncheck_other_when_checked(radio_element, elements_clone.clone());
                    subscriptions_clone.borrow_mut().push(subscription);
                }
            }));

        RadioController {
            _elements: elements,
            _subscriptions: subscriptions,
            _phantom_data: PhantomData,
        }
    }

    fn uncheck_other_when_checked(
        element: Rc<RefCell<dyn ControlObject>>,
        elements: Rc<RefCell<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>,
    ) -> Subscription {
        let element_clone = element.clone();
        element
            .borrow()
            .as_any()
            .downcast_ref::<R>()
            .unwrap()
            .on_checked(Box::new(move || {
                for el in elements.borrow_mut().deref() {
                    if !Rc::ptr_eq(&el, &element_clone) {
                        el.borrow_mut()
                            .deref_mut()
                            .as_any_mut()
                            .downcast_mut::<R>()
                            .unwrap()
                            .set_is_checked(false);
                    }
                }
            }))
    }
}
