use std::cell::RefCell;
use std::rc::Rc;

pub struct ObservableVec<T> {
    items: Vec<Rc<RefCell<T>>>,
}

impl<T> ObservableVec<T> {
    pub fn empty() -> Self {
        ObservableVec { items: Vec::new() }
    }

    pub fn new<I: IntoIterator<Item = Rc<RefCell<T>>>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for i in iter {
            vec.push(i);
        }
        ObservableVec { items: vec }
    }
}

impl<'a, T> IntoIterator for &'a ObservableVec<T> {
    type Item = &'a Rc<RefCell<T>>;
    type IntoIter = ::std::slice::Iter<'a, Rc<RefCell<T>>>;

    fn into_iter(self) -> ::std::slice::Iter<'a, Rc<RefCell<T>>> {
        self.items.iter()
    }
}
