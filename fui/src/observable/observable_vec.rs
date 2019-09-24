use std::iter::FromIterator;

pub struct ObservableVec<T> {
    items: Vec<T>,
}

impl<T> ObservableVec<T> {
    pub fn new() -> Self {
        ObservableVec { items: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    pub fn remove_filter<F>(&mut self, mut filter: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mut i = 0;
        while i != self.items.len() {
            if filter(&mut self.items[i]) {
                self.items.remove(i);
                println!("Removed {}!", i);
            } else {
                i += 1;
            }
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
        ObservableVec { items: vec }
    }
}
