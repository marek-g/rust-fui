use std::cell::RefCell;
use std::rc::Rc;

use crate::control::ControlObject;
use crate::observable::Event;
use crate::observable::EventSubscription;
use crate::observable::ObservableChangedEventArgs;
use crate::observable::ObservableVec;
use crate::{observable::ObservableCollectionExt, ObservableCollectionMap, ViewModelObject};
use crate::{view::ViewModel, ObservableCollection, Property};

///
/// Converts vector of view models to observable collection.
///
impl<V: ViewModel + 'static> From<&Vec<Rc<RefCell<V>>>>
    for Box<dyn ObservableCollection<Box<(dyn ViewModelObject)>>>
{
    fn from(collection: &Vec<Rc<RefCell<V>>>) -> Self {
        Box::new(
            collection
                .iter()
                .map(|el| Box::new(el.clone()) as Box<dyn ViewModelObject>)
                .collect::<Vec<_>>(),
        ) as Box<dyn ObservableCollection<Box<dyn ViewModelObject>>>
    }
}

impl<V: ViewModel + 'static> From<Vec<Rc<RefCell<V>>>>
    for Box<dyn ObservableCollection<Box<(dyn ViewModelObject)>>>
{
    fn from(collection: Vec<Rc<RefCell<V>>>) -> Self {
        Box::new(
            collection
                .into_iter()
                .map(|el| Box::new(el) as Box<dyn ViewModelObject>)
                .collect::<Vec<_>>(),
        ) as Box<dyn ObservableCollection<Box<dyn ViewModelObject>>>
    }
}

impl<V: ViewModel + 'static> From<Vec<Rc<RefCell<V>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<V>>>>
{
    fn from(collection: Vec<Rc<RefCell<V>>>) -> Self {
        Box::new(collection) as Box<dyn ObservableCollection<Rc<RefCell<V>>>>
    }
}

impl<V> From<&ObservableVec<Rc<RefCell<V>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
where
    V: ViewModel,
{
    fn from(src: &ObservableVec<Rc<RefCell<V>>>) -> Self {
        Box::new(
            (src as &dyn ObservableCollection<Rc<RefCell<V>>>).map(|vm| ViewModel::create_view(vm)),
        )
    }
}

impl From<&ObservableVec<Rc<RefCell<dyn ControlObject>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
{
    fn from(src: &ObservableVec<Rc<RefCell<dyn ControlObject>>>) -> Self {
        Box::new(
            (src as &dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>).map(|el| el.clone()),
        )
    }
}

impl<V> From<&ObservableCollectionMap<Rc<RefCell<V>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
where
    V: 'static + ViewModel,
{
    fn from(src: &ObservableCollectionMap<Rc<RefCell<V>>>) -> Self {
        Box::new(
            (src as &dyn ObservableCollection<Rc<RefCell<V>>>).map(|vm| ViewModel::create_view(vm)),
        )
    }
}

impl<V> From<&Box<dyn ObservableCollection<Rc<RefCell<V>>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
where
    V: 'static + ViewModel,
{
    fn from(src: &Box<dyn ObservableCollection<Rc<RefCell<V>>>>) -> Self {
        Box::new(src.map(|vm| ViewModel::create_view(vm)))
    }
}

impl<V> From<&Rc<RefCell<dyn ObservableCollection<Rc<RefCell<V>>>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
where
    V: 'static + ViewModel,
{
    fn from(src: &Rc<RefCell<dyn ObservableCollection<Rc<RefCell<V>>>>>) -> Self {
        Box::new(src.borrow().map(|vm| ViewModel::create_view(vm)))
    }
}

impl From<&Rc<RefCell<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
{
    fn from(src: &Rc<RefCell<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>) -> Self {
        Box::new(src.borrow().map(|el| el.clone()))
    }
}

impl From<&Property<Rc<RefCell<dyn ControlObject>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
{
    fn from(src: &Property<Rc<RefCell<dyn ControlObject>>>) -> Self {
        Box::new(Property::binded_from(&src))
    }
}

impl From<&Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
{
    fn from(src: &Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>) -> Self {
        Box::new(Property::binded_from(&src.borrow()))
    }
}

impl<V> From<&Property<Rc<RefCell<V>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
where
    V: 'static + ViewModel + PartialEq,
{
    fn from(src: &Property<Rc<RefCell<V>>>) -> Self {
        Box::new(
            (src as &dyn ObservableCollection<Rc<RefCell<V>>>).map(|vm| ViewModel::create_view(vm)),
        )
    }
}

impl<V> From<&Property<Option<Rc<RefCell<V>>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
where
    V: 'static + ViewModel + PartialEq,
{
    fn from(src: &Property<Option<Rc<RefCell<V>>>>) -> Self {
        Box::new(
            (src as &dyn ObservableCollection<Rc<RefCell<V>>>).map(|vm| ViewModel::create_view(vm)),
        )
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
    pub fn new(
        sources: Vec<Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>,
    ) -> Self {
        let changed_event = Rc::new(RefCell::new(Event::new()));
        let mut source_changed_event_subscriptions = Vec::new();
        for source in &sources {
            let changed_event_clone = changed_event.clone();
            let handler =
                Box::new(move |changed_args| changed_event_clone.borrow().emit(changed_args));
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

    fn on_changed(
        &self,
        f: Box<dyn Fn(ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>)>,
    ) -> Option<EventSubscription> {
        Some(
            self.changed_event
                .borrow_mut()
                .subscribe(move |args| f(args)),
        )
    }
}
