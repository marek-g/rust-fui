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
impl<V: ViewModel + 'static> From<Vec<Rc<V>>> for Box<dyn ObservableCollection<Rc<V>>> {
    fn from(collection: Vec<Rc<V>>) -> Self {
        Box::new(collection) as Box<dyn ObservableCollection<Rc<V>>>
    }
}

impl<V: ViewModel + 'static> From<&Vec<Rc<V>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
{
    fn from(src: &Vec<Rc<V>>) -> Self {
        Box::new(src.map(|vm| ViewModel::create_view(vm)))
    }
}

///
/// Converts ObservableVec of view models to observable collection.
///
impl<V> From<&ObservableVec<Rc<V>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
where
    V: 'static + ViewModel,
{
    fn from(src: &ObservableVec<Rc<V>>) -> Self {
        Box::new(src.map(|vm| ViewModel::create_view(vm)))
    }
}

impl From<&ObservableVec<Rc<dyn ControlObject>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
{
    fn from(src: &ObservableVec<Rc<dyn ControlObject>>) -> Self {
        Box::new(src.map(|el| el.clone()))
    }
}

///
/// Converts ObservableCollectionMap to observable collection.
///

impl<V> From<ObservableCollectionMap<Rc<V>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
where
    V: 'static + ViewModel,
{
    fn from(src: ObservableCollectionMap<Rc<V>>) -> Self {
        Box::new(src.map(|vm| ViewModel::create_view(vm)))
    }
}

impl From<ObservableCollectionMap<Rc<dyn ControlObject>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
{
    fn from(src: ObservableCollectionMap<Rc<dyn ControlObject>>) -> Self {
        Box::new(src)
    }
}

impl<V> From<&ObservableCollectionMap<Rc<V>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
where
    V: 'static + ViewModel,
{
    fn from(src: &ObservableCollectionMap<Rc<V>>) -> Self {
        Box::new(src.map(|vm| ViewModel::create_view(vm)))
    }
}

impl From<&ObservableCollectionMap<Rc<dyn ControlObject>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
{
    fn from(src: &ObservableCollectionMap<Rc<dyn ControlObject>>) -> Self {
        Box::new(src.map(|el| el.clone()))
    }
}

///
/// Converts ObservableCollectionFlatMap to observable collection.
///

impl From<ObservableCollectionFlatMap<Rc<dyn ControlObject>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
{
    fn from(src: ObservableCollectionFlatMap<Rc<dyn ControlObject>>) -> Self {
        Box::new(src)
    }
}

impl From<&ObservableCollectionFlatMap<Rc<dyn ControlObject>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
{
    fn from(src: &ObservableCollectionFlatMap<Rc<dyn ControlObject>>) -> Self {
        Box::new(src.map(|el| el.clone()))
    }
}

///
/// Converts Property to observable collection.
///

impl From<&Property<Rc<dyn ControlObject>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
{
    fn from(src: &Property<Rc<dyn ControlObject>>) -> Self {
        Box::new(src.clone())
    }
}

impl<V> From<&Property<Rc<V>>> for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
where
    V: 'static + ViewModel + PartialEq,
{
    fn from(src: &Property<Rc<V>>) -> Self {
        Box::new(src.map(|vm| ViewModel::create_view(vm)))
    }
}

impl<V> From<&Property<Option<Rc<V>>>>
    for Box<dyn ObservableCollection<Rc<dyn ControlObject>>>
where
    V: 'static + ViewModel + PartialEq,
{
    fn from(src: &Property<Option<Rc<V>>>) -> Self {
        Box::new(src.map(|vm| ViewModel::create_view(vm)))
    }
}
