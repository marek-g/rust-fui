use std::cell::RefCell;
use std::cell::RefMut;
use std::ops::Index;
use std::rc::Rc;

use crate::control::ControlObject;
use crate::observable::Event;
use crate::observable::EventSubscription;
use crate::observable::ObservableChangedEventArgs;
use crate::observable::ObservableVec;
use crate::{Property, view::ViewModel, ObservableCollection};

///
/// DynamicChildrenSource.
///
pub struct DynamicChildrenSource {
    children: Rc<RefCell<Vec<Rc<RefCell<dyn ControlObject>>>>>,
    changed_event: Rc<RefCell<Event<ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>>>>,
    _children_changed_event_subscription: EventSubscription,
}

impl ObservableCollection<Rc<RefCell<dyn ControlObject>>> for DynamicChildrenSource {
    fn len(&self) -> usize {
        self.children.borrow().len()
    }

    fn get(&self, index: usize) -> Rc<RefCell<dyn ControlObject>> {
        self.children.borrow().index(index).clone()
    }

    fn get_changed_event(&self) -> Option<RefMut<'_, Event<ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>>>> {
        Some(self.changed_event.borrow_mut())
    }
}

impl<T> From<&ObservableVec<Rc<RefCell<T>>>> for DynamicChildrenSource
    where T: ViewModel {
    fn from(children: &ObservableVec<Rc<RefCell<T>>>) -> Self {
        let children_vec = children
            .into_iter()
            .map(|vm| ViewModel::to_view(&vm))
            .collect();

        let children_rc = Rc::new(RefCell::new(children_vec));
        let changed_event_rc = Rc::new(RefCell::new(Event::new()));

        let children_rc_clone = children_rc.clone();
        let changed_event_rc_clone = changed_event_rc.clone();
        let event_subscription =
            children
                .get_changed_event()
                .subscribe(move |changed_args| match changed_args {
                    ObservableChangedEventArgs::Insert { index, value } => {
                        let mut vec: RefMut<'_, Vec<Rc<RefCell<dyn ControlObject>>>> =
                            children_rc_clone.borrow_mut();
                        let control = ViewModel::to_view(&value);
                        let control_clone = control.clone();
                        vec.insert(index, control);

                        changed_event_rc_clone
                            .borrow()
                            .emit(ObservableChangedEventArgs::Insert { index, value: control_clone });
                    }
                    ObservableChangedEventArgs::Remove {
                        index,
                        value: _value,
                    } => {
                        let mut vec: RefMut<'_, Vec<Rc<RefCell<dyn ControlObject>>>> =
                            children_rc_clone.borrow_mut();
                        let control = vec.remove(index);

                        changed_event_rc_clone
                            .borrow()
                            .emit(ObservableChangedEventArgs::Remove { index, value: control });
                    }
                });

        DynamicChildrenSource {
            children: children_rc,
            changed_event: changed_event_rc,
            _children_changed_event_subscription: event_subscription,
        }
    }
}

impl From<&Property<Rc<RefCell<dyn ControlObject>>>> for DynamicChildrenSource {
    fn from(child: &Property<Rc<RefCell<dyn ControlObject>>>) -> Self {
        let children_rc = Rc::new(RefCell::new(
            vec![child.get()]));
        let changed_event_rc = Rc::new(RefCell::new(Event::new()));

        let children_rc_clone = children_rc.clone();
        let changed_event_rc_clone = changed_event_rc.clone();
        let event_subscription =
            child.on_changed(move |new_control| {
                let mut vec = children_rc_clone.borrow_mut();

                let old_control = vec.pop();
                if let Some(old_control) = old_control {
                    changed_event_rc_clone.borrow().emit(
                        ObservableChangedEventArgs::Remove { index: 0, value: old_control }
                    );
                }

                vec.push(new_control.clone());
                changed_event_rc_clone.borrow().emit(
                    ObservableChangedEventArgs::Insert { index: 0, value: new_control }
                );
            });

        DynamicChildrenSource {
            children: children_rc,
            changed_event: changed_event_rc,
            _children_changed_event_subscription: event_subscription,
        }
    }
}

impl From<&Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>> for DynamicChildrenSource {
    fn from(child: &Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>) -> Self {
        let children_rc = Rc::new(RefCell::new(
            vec![child.borrow().get()]));
        let changed_event_rc = Rc::new(RefCell::new(Event::new()));

        let children_rc_clone = children_rc.clone();
        let changed_event_rc_clone = changed_event_rc.clone();
        let event_subscription =
            child.borrow().on_changed(move |new_control| {
                let mut vec = children_rc_clone.borrow_mut();

                let old_control = vec.pop();
                if let Some(old_control) = old_control {
                    changed_event_rc_clone.borrow().emit(
                        ObservableChangedEventArgs::Remove { index: 0, value: old_control }
                    );
                }

                vec.push(new_control.clone());
                changed_event_rc_clone.borrow().emit(
                    ObservableChangedEventArgs::Insert { index: 0, value: new_control }
                );
            });

        DynamicChildrenSource {
            children: children_rc,
            changed_event: changed_event_rc,
            _children_changed_event_subscription: event_subscription,
        }
    }
}

///
/// AggregatedChildrenSource.
///
pub struct AggregatedChildrenSource {
    sources: Vec<Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>,
    changed_event: Option<Rc<RefCell<Event<ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>>>>>,
    source_changed_event_subscriptions: Vec<EventSubscription>,
}

impl AggregatedChildrenSource {
    pub fn new(sources: Vec<Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>) -> Self {
        let mut changed_event = None;
        let mut source_changed_event_subscriptions = Vec::new();
        for source in &sources {
            if let Some(mut source_changed_event) = source.get_changed_event() {
                let dest_event_rc = changed_event
                    .get_or_insert_with(|| Rc::new(RefCell::new(Event::new())))
                    .clone();
                source_changed_event_subscriptions.push(
                    source_changed_event
                        .subscribe(move |changed_args| dest_event_rc.borrow().emit(changed_args)),
                );
            }
        }
        AggregatedChildrenSource {
            sources,
            changed_event,
            source_changed_event_subscriptions,
        }
    }
}

impl ObservableCollection<Rc<RefCell<dyn ControlObject>>> for AggregatedChildrenSource {
    fn len(&self) -> usize {
        self.sources.iter().map(|s| s.len()).sum()
    }

    fn get(&self, index: usize) -> Rc<RefCell<dyn ControlObject>> {
        let mut new_index = index;
        for source in &self.sources {
            let len = source.len();
            if new_index < len {
                return source.get(new_index);
            } else {
                new_index -= len;
            }
        }

        panic!(format!(
            "index out of bounds: the len is {} but the index is {}",
            self.len(),
            index
        ))
    }

    fn get_changed_event(&self) -> Option<RefMut<'_, Event<ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>>>> {
        match &self.changed_event {
            None => None,
            Some(ref changed_event) => Some(changed_event.borrow_mut()),
        }
    }
}
