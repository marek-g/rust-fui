use crate::Subscription;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VecDiff<A> {
    Clear {},

    InsertAt { index: usize, value: A },

    RemoveAt { index: usize },

    Move { old_index: usize, new_index: usize },

    Pop {},

    Push { value: A },
}

pub trait ObservableCollection<T: 'static + Clone> {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<T>;
    fn on_changed(&self, f: Box<dyn FnMut(VecDiff<T>)>) -> Option<Subscription>;
}

///
/// ObservableCollectionIterator.
///
pub struct ObservableCollectionIterator<'a, T>
where
    T: 'static + Clone,
{
    source: &'a dyn ObservableCollection<T>,
    pos: usize,
    len: usize,
}

impl<'a, T> Iterator for ObservableCollectionIterator<'a, T>
where
    T: 'static + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.pos < self.len {
            self.pos += 1;
            self.source.get(self.pos - 1)
        } else {
            None
        }
    }
}

impl<'a, T> DoubleEndedIterator for ObservableCollectionIterator<'a, T>
where
    T: 'static + Clone,
{
    fn next_back(&mut self) -> Option<T> {
        if self.len > self.pos {
            self.len -= 1;
            self.source.get(self.len)
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a dyn ObservableCollection<T>
where
    T: 'static + Clone,
{
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
/// ObservableCollection for Vec.
///
impl<T> ObservableCollection<T> for Vec<T>
where
    T: 'static + Clone,
{
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn get(&self, index: usize) -> Option<T> {
        self.as_slice().get(index).map(|el| el.clone())
    }

    fn on_changed(&self, _: Box<dyn FnMut(VecDiff<T>)>) -> Option<Subscription> {
        None
    }
}
