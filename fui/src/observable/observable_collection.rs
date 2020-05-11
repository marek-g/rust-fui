use std::cell::RefMut;
use crate::Event;

#[derive(Clone)]
pub enum ObservableChangedEventArgs<T: 'static + Clone> {
    Insert { index: usize, value: T },
    Remove { index: usize, value: T },
}

pub trait ObservableCollection<T: 'static + Clone> {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> T;
    fn get_changed_event(&self) -> Option<RefMut<'_, Event<ObservableChangedEventArgs<T>>>>;
}
