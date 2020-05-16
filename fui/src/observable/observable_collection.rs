use std::cell::RefMut;
use std::cell::RefCell;
use std::{ops::Index, rc::Rc};
use crate::{EventSubscription, Event, ObservableVec, Property};

#[derive(Clone)]
pub enum ObservableChangedEventArgs<T: 'static + Clone> {
    Insert { index: usize, value: T },
    Remove { index: usize },
}

pub trait ObservableCollection<T: 'static + Clone> {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> T;
    fn on_changed(&self, f: Box<dyn Fn(ObservableChangedEventArgs<T>)>) -> Option<EventSubscription>;
}

///
/// ObservableCollectionIterator.
///
pub struct ObservableCollectionIterator<'a, T>
    where T: 'static + Clone {
    source: &'a dyn ObservableCollection<T>,
    pos: usize,
    len: usize,
}

impl<'a, T> Iterator for ObservableCollectionIterator<'a, T>
    where T: 'static + Clone {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.pos < self.len {
            self.pos += 1;
            Some(self.source.get(self.pos - 1))
        } else {
            None
        }
    }
}

impl<'a, T> DoubleEndedIterator for ObservableCollectionIterator<'a, T>
    where T: 'static + Clone {
    fn next_back(&mut self) -> Option<T> {
        if self.len > self.pos {
            self.len -= 1;
            Some(self.source.get(self.len))
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a dyn ObservableCollection<T>
    where T: 'static + Clone {
    type Item = T;
    type IntoIter = ObservableCollectionIterator<'a, T>;

    fn into_iter(self) -> ObservableCollectionIterator<'a, T> {
        ObservableCollectionIterator {
            source: self,
            pos: 0,
            len: self.len(),
        }
    }
}

///
/// ObservableCollectionMap.
///
pub struct ObservableCollectionMap<TDst: 'static + Clone> {
    items: Rc<RefCell<Vec<TDst>>>,
    changed_event: Rc<RefCell<Event<ObservableChangedEventArgs<TDst>>>>,
    _items_changed_event_subscription: Option<EventSubscription>,
}

impl<T: 'static + Clone> ObservableCollection<T> for ObservableCollectionMap<T> {
    fn len(&self) -> usize {
        self.items.borrow().len()
    }

    fn get(&self, index: usize) -> T {
        self.items.borrow().index(index).clone()
    }

    fn on_changed(&self, f: Box<dyn Fn(ObservableChangedEventArgs<T>)>) -> Option<EventSubscription> {
        Some(self.changed_event.borrow_mut().subscribe(f))
    }
}

pub trait ObservableCollectionExt<T: 'static + Clone> {
    fn map<TDst, F>(&self, f: F) -> ObservableCollectionMap<TDst>
        where TDst: 'static + Clone, F: 'static + Fn(&T) -> TDst;
}

impl<T: 'static + Clone> ObservableCollectionExt<T> for dyn ObservableCollection<T> {
    fn map<TDst, F>(&self, f: F) -> ObservableCollectionMap<TDst>
        where TDst: 'static + Clone, F: 'static + Fn(&T) -> TDst {
        let items_vec = self
            .into_iter()
            .map(|item| f(&item))
            .collect();

        let items_rc = Rc::new(RefCell::new(items_vec));
        let changed_event_rc = Rc::new(RefCell::new(Event::new()));

        let items_rc_clone = items_rc.clone();
        let changed_event_rc_clone = changed_event_rc.clone();
        let handler = Box::new(
            move |changed_args| match changed_args {
                ObservableChangedEventArgs::Insert { index, value } => {
                    let mut vec: RefMut<'_, Vec<TDst>> =
                        items_rc_clone.borrow_mut();
                    let new_item = f(&value);
                    let new_item_clone = new_item.clone();
                    vec.insert(index, new_item);

                    changed_event_rc_clone
                        .borrow()
                        .emit(ObservableChangedEventArgs::Insert { index, value: new_item_clone });
                }

                ObservableChangedEventArgs::Remove {
                    index,
                } => {
                    let mut vec: RefMut<'_, Vec<TDst>> = items_rc_clone.borrow_mut();
                    vec.remove(index);

                    changed_event_rc_clone
                        .borrow()
                        .emit(ObservableChangedEventArgs::Remove { index });
                }
            }
        );
        let event_subscription =
            self
                .on_changed(handler);

        ObservableCollectionMap {
            items: items_rc,
            changed_event: changed_event_rc,
            _items_changed_event_subscription: event_subscription,
        }
    }
}

///
/// ObservableCollection for Vec.
///
impl<T> ObservableCollection<T> for Vec<T>
    where T: 'static + Clone {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn get(&self, index: usize) -> T {
        Vec::index(self, index).clone()
    }

    fn on_changed(&self, _: Box<dyn Fn(ObservableChangedEventArgs<T>)>) -> Option<EventSubscription> {
        None
    }
}

///
/// ObservableCollection for ObservableVec.
///
impl<T> ObservableCollection<T> for ObservableVec<T>
    where T: 'static + Clone {
    fn len(&self) -> usize {
        ObservableVec::len(self)
    }

    fn get(&self, index: usize) -> T {
        ObservableVec::get(self, index)
    }

    fn on_changed(&self, f: Box<dyn Fn(ObservableChangedEventArgs<T>)>) -> Option<EventSubscription> {
        Some(ObservableVec::on_changed(self, f))
    }
}

///
/// ObservableCollection for Property.
///
impl<T> ObservableCollection<T> for Property<T>
    where T: 'static + Clone + PartialEq {
    fn len(&self) -> usize {
        1
    }

    fn get(&self, _index: usize) -> T {
        Property::get(self)
    }

    fn on_changed(&self, f: Box<dyn Fn(ObservableChangedEventArgs<T>)>) -> Option<EventSubscription> {
        Some(Property::on_changed(self, move |v| {
            f(ObservableChangedEventArgs::Remove { index: 0 });
            f(ObservableChangedEventArgs::Insert { index: 0, value: v });
        }))
    }
}
