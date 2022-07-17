use std::cell::RefCell;
use std::rc::Rc;

use crate::control::ControlObject;
use crate::observable::ObservableVec;
use crate::{
    observable::ObservableCollectionExt, ObservableCollectionFlatMap, ObservableCollectionMap,
};
use crate::{view::ViewModel, ObservableCollection, Property};

///
/// Converts vector of view models to observable collection.
///
impl<V: ViewModel + 'static> From<Vec<Rc<RefCell<V>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<V>>>>
{
    fn from(collection: Vec<Rc<RefCell<V>>>) -> Self {
        Box::new(collection) as Box<dyn ObservableCollection<Rc<RefCell<V>>>>
    }
}

impl<V: ViewModel + 'static> From<&Vec<Rc<RefCell<V>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
{
    fn from(src: &Vec<Rc<RefCell<V>>>) -> Self {
        Box::new(
            (src as &dyn ObservableCollection<Rc<RefCell<V>>>).map(|vm| ViewModel::create_view(vm)),
        )
    }
}

///
/// Converts ObservableVec of view models to observable collection.
///
impl<V> From<&ObservableVec<Rc<RefCell<V>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
where
    V: 'static + ViewModel,
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

///
/// Converts ObservableCollectionMap to observable collection.
///

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

impl From<&ObservableCollectionMap<Rc<RefCell<dyn ControlObject>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
{
    fn from(src: &ObservableCollectionMap<Rc<RefCell<dyn ControlObject>>>) -> Self {
        Box::new(
            (src as &dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>)
                .map(|view| view.clone()),
        )
    }
}

///
/// Converts ObservableCollectionFlatMap to observable collection.
///

impl From<ObservableCollectionFlatMap<Rc<RefCell<dyn ControlObject>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
{
    fn from(src: ObservableCollectionFlatMap<Rc<RefCell<dyn ControlObject>>>) -> Self {
        Box::new(src)
    }
}

///
/// Converts Property to observable collection.
///

impl From<&Property<Rc<RefCell<dyn ControlObject>>>>
    for Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>
{
    fn from(src: &Property<Rc<RefCell<dyn ControlObject>>>) -> Self {
        Box::new(src.clone())
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
