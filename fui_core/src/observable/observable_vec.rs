use std::{borrow::Borrow, iter::FromIterator};

use futures_signals::signal_vec::{MutableVec, MutableVecLockMut, MutableVecLockRef, SignalVecExt};

use crate::{spawn_local, ObservableCollection, Subscription, VecDiff};

#[derive(Clone)]
pub struct ObservableVec<T: 'static + Clone> {
    items: MutableVec<T>,
}

impl<T: 'static + Clone> ObservableVec<T> {
    pub fn new() -> Self {
        ObservableVec {
            items: MutableVec::new(),
        }
    }

    pub fn lock_ref(&self) -> MutableVecLockRef<'_, T> {
        self.items.lock_ref()
    }

    pub fn lock_mut(&self) -> MutableVecLockMut<'_, T> {
        self.items.lock_mut()
    }

    pub fn len(&self) -> usize {
        self.items.lock_ref().len()
    }

    pub fn get(&self, index: usize) -> Option<T> {
        self.items.lock_ref().get(index).map(|el| el.clone())
    }

    pub fn on_changed<F>(&self, mut f: F) -> Subscription
    where
        F: 'static + FnMut(VecDiff<T>),
    {
        let mut mirror: Vec<T> = self.items.lock_ref().to_vec();

        let future = self.items.borrow().signal_vec_cloned().for_each(move |v| {
            match v {
                futures_signals::signal_vec::VecDiff::Replace { values } => {
                    let old_values = std::mem::replace(&mut mirror, values.clone());
                    f(VecDiff::Clear { values: old_values });
                    mirror.iter().enumerate().for_each(|(index, value)| {
                        f(VecDiff::InsertAt {
                            index,
                            value: value.clone(),
                        })
                    });
                }
                futures_signals::signal_vec::VecDiff::InsertAt { index, value } => {
                    mirror.insert(index, value.clone());
                    f(VecDiff::InsertAt { index, value })
                }
                futures_signals::signal_vec::VecDiff::UpdateAt { index, value } => {
                    let old_value = std::mem::replace(&mut mirror[index], value.clone());
                    f(VecDiff::RemoveAt {
                        index,
                        value: old_value,
                    });
                    f(VecDiff::InsertAt { index, value });
                }
                futures_signals::signal_vec::VecDiff::RemoveAt { index } => {
                    let value = mirror.remove(index);
                    f(VecDiff::RemoveAt { index, value })
                }
                futures_signals::signal_vec::VecDiff::Move {
                    old_index,
                    new_index,
                } => {
                    let value = mirror.remove(old_index);
                    mirror.insert(new_index, value.clone());
                    f(VecDiff::Move {
                        old_index,
                        new_index,
                    });
                }
                futures_signals::signal_vec::VecDiff::Push { value } => {
                    mirror.push(value.clone());
                    f(VecDiff::Push { value })
                }
                futures_signals::signal_vec::VecDiff::Pop {} => {
                    let value = mirror.pop().unwrap();
                    f(VecDiff::Pop { value })
                }
                futures_signals::signal_vec::VecDiff::Clear {} => {
                    let old_values = std::mem::replace(&mut mirror, Vec::new());
                    f(VecDiff::Clear { values: old_values });
                }
            }
            async {}
        });
        Subscription::SpawnLocal(spawn_local(future))
    }

    pub fn push(&self, value: T) {
        self.items.lock_mut().push_cloned(value);
    }

    pub fn clear(&self) {
        self.items.lock_mut().clear();
    }

    pub fn retain<F>(&self, filter: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.items.lock_mut().retain(filter);
    }
}

impl<T: 'static + Clone> FromIterator<T> for ObservableVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for i in iter {
            vec.push(i);
        }
        ObservableVec {
            items: MutableVec::new_with_values(vec),
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
        Some(ObservableVec::on_changed(self, f))
    }
}
