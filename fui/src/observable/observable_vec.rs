use observable::event::Event;
use std::iter::FromIterator;

pub struct ObservableVec<T> {
    items: Vec<T>,
    changed_event: Event<()>,
}

impl<T> ObservableVec<T> {
    pub fn new() -> Self {
        ObservableVec {
            items: Vec::new(),
            changed_event: Event::new(),
        }
    }

    pub fn get_changed_event(&mut self) -> &mut Event<()> {
        &mut self.changed_event
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value);
        self.changed_event.emit(());
    }

    pub fn remove_filter<F>(&mut self, mut filter: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mut i = 0;
        let mut removed = false;
        while i != self.items.len() {
            if filter(&mut self.items[i]) {
                self.items.remove(i);
                removed = true;
                println!("Removed {}!", i);
            } else {
                i += 1;
            }
        }

        if removed {
            self.changed_event.emit(());
        }
    }
}

impl<'a, T> IntoIterator for &'a ObservableVec<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> ::std::slice::Iter<'a, T> {
        self.items.iter()
    }
}

impl<T> FromIterator<T> for ObservableVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for i in iter {
            vec.push(i);
        }
        ObservableVec {
            items: vec,
            changed_event: Event::new(),
        }
    }
}
