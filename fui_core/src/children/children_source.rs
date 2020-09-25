use std::cell::RefCell;
use std::rc::Rc;

use crate::control::ControlObject;
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
