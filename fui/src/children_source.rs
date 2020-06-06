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
use crate::{ObservableCollectionMap, observable::ObservableCollectionExt};

/*impl<T> Into<Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>> for &ObservableVec<Rc<RefCell<T>>>
    where T: ViewModel {
    fn into(self) -> Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>> {
        Box::new((self as &dyn ObservableCollection<Rc<RefCell<T>>>).map(|vm| { ViewModel::to_view(vm) }))
    }
}*/

impl<T> From<&ObservableVec<Rc<RefCell<T>>>> for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
    where T: ViewModel {
    fn from(src: &ObservableVec<Rc<RefCell<T>>>) -> Self {
        Box::new((src as &dyn ObservableCollection<Rc<RefCell<T>>>).map(|vm| { ViewModel::to_view(vm) }))
    }
}

/*impl From<&Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>> for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>> {
    fn from(src: &Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>) -> Self {
        Box::new((src as &dyn ObservableCollection<Rc<RefCell<T>>>).map(|vm| { ViewModel::to_view(vm) }))
    }
}*/

impl<T> From<&ObservableCollectionMap<Rc<RefCell<T>>>> for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
    where T: 'static + ViewModel {
    fn from(src: &ObservableCollectionMap<Rc<RefCell<T>>>) -> Self {
        Box::new((src as &dyn ObservableCollection<Rc<RefCell<T>>>).map(|vm| { ViewModel::to_view(vm) }))
    }
}

impl<T> From<&Box<dyn ObservableCollection<Rc<RefCell<T>>>>> for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
    where T: 'static + ViewModel {
    fn from(src: &Box<dyn ObservableCollection<Rc<RefCell<T>>>>) -> Self {
        Box::new(src.map(|vm| { ViewModel::to_view(vm) }))
    }
}

impl From<&Property<Rc<RefCell<dyn ControlObject>>>> for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>> {
    fn from(src: &Property<Rc<RefCell<dyn ControlObject>>>) -> Self {
        Box::new(Property::binded_from(&src))
    }
}

impl From<&Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>> for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>> {
    fn from(src: &Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>) -> Self {
        Box::new(Property::binded_from(&src.borrow()))
    }
}

///
/// AggregatedChildrenSource.
///
pub struct AggregatedChildrenSource {
    sources: Vec<Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>,
    changed_event: Rc<RefCell<Event<ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>>>>,
    source_changed_event_subscriptions: Vec<EventSubscription>,
}

impl AggregatedChildrenSource {
    pub fn new(sources: Vec<Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>) -> Self {
        let mut changed_event = Rc::new(RefCell::new(Event::new()));
        let mut source_changed_event_subscriptions = Vec::new();
        for source in &sources {
            let changed_event_clone = changed_event.clone();
            let handler = Box::new(move |changed_args| changed_event_clone.borrow().emit(changed_args));
            if let Some(subscription) = source.on_changed(handler) {
                source_changed_event_subscriptions.push(subscription);
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

    fn on_changed(&self, f: Box<dyn Fn(ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>)>) -> Option<EventSubscription> {
        Some(self.changed_event.borrow_mut().subscribe(move |args| f(args)))
    }
}
