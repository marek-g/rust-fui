use std::cell::RefCell;
use std::iter::FromIterator;

use crate::{
    observable::event::Event, EventSubscription, ObservableCollection, Subscription, VecDiff,
};

pub struct ObservableVec<T: 'static + Clone> {
    items: Vec<T>,
    changed_event: RefCell<Event<VecDiff<T>>>,
}

impl<T: 'static + Clone> ObservableVec<T> {
    pub fn new() -> Self {
        ObservableVec {
            items: Vec::new(),
            changed_event: RefCell::new(Event::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<T> {
        self.items.get(index).map(|el| el.clone())
    }

    pub fn on_changed<F>(&self, f: F) -> EventSubscription
    where
        F: 'static + FnMut(VecDiff<T>),
    {
        self.changed_event.borrow_mut().subscribe(f)
    }

    pub fn push(&mut self, value: T) {
        let event_args = VecDiff::InsertAt {
            index: self.items.len(),
            value: value.clone(),
        };
        self.items.push(value);
        self.changed_event.borrow().emit(event_args);
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.changed_event.borrow().emit(VecDiff::Clear {});
    }

    pub fn remove_filter<F>(&mut self, mut filter: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mut i = 0;
        while i != self.items.len() {
            if filter(&mut self.items[i]) {
                let event_args = VecDiff::RemoveAt { index: i };
                self.items.remove(i);
                self.changed_event.borrow().emit(event_args);
            } else {
                i += 1;
            }
        }
    }
}

impl<'a, T: 'static + Clone> IntoIterator for &'a ObservableVec<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> ::std::slice::Iter<'a, T> {
        (&self.items as &[T]).iter()
    }
}

impl<T: 'static + Clone> FromIterator<T> for ObservableVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for i in iter {
            vec.push(i);
        }
        ObservableVec {
            items: vec,
            changed_event: RefCell::new(Event::new()),
        }
    }
}

///
/// ObservableCollection for ObservableVec.
///
impl<T> ObservableCollection<T> for ObservableVec<T>
where
    T: 'static + Clone,
{
    fn len(&self) -> usize {
        ObservableVec::len(self)
    }

    fn get(&self, index: usize) -> Option<T> {
        ObservableVec::get(self, index)
    }

    fn on_changed(&self, f: Box<dyn FnMut(VecDiff<T>)>) -> Option<Subscription> {
        Some(Subscription::EventSubscription(ObservableVec::on_changed(
            self, f,
        )))
    }
}
