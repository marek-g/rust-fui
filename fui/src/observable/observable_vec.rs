use std::cell::RefCell;
use std::cell::RefMut;
use std::iter::FromIterator;

use crate::observable::event::Event;

#[derive(Clone)]
pub enum ObservableChangedEventArgs<T: 'static + Clone> {
    Insert { index: usize, value: T },
    Remove { index: usize, value: T },
}

pub struct ObservableVec<T: 'static + Clone> {
    items: Vec<T>,
    changed_event: RefCell<Event<ObservableChangedEventArgs<T>>>,
}

impl<T: 'static + Clone> ObservableVec<T> {
    pub fn new() -> Self {
        ObservableVec {
            items: Vec::new(),
            changed_event: RefCell::new(Event::new()),
        }
    }

    pub fn get_changed_event(&self) -> RefMut<'_, Event<ObservableChangedEventArgs<T>>> {
        self.changed_event.borrow_mut()
    }

    pub fn push(&mut self, value: T) {
        let event_args = ObservableChangedEventArgs::Insert {
            index: self.items.len(),
            value: value.clone(),
        };
        self.items.push(value);
        self.changed_event.borrow().emit(event_args);
    }

    pub fn remove_filter<F>(&mut self, mut filter: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mut i = 0;
        while i != self.items.len() {
            if filter(&mut self.items[i]) {
                let event_args = ObservableChangedEventArgs::Remove {
                    index: i,
                    value: self.items[i].clone(),
                };
                self.items.remove(i);
                self.changed_event.borrow().emit(event_args);
                println!("Removed {}!", i);
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
        self.items.iter()
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
