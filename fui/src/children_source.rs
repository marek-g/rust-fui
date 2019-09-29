use observable::ChangedEventArgs;
use observable::EventSubscription;
use observable::ObservableVec;
use std::cell::RefCell;
use std::cell::RefMut;
use std::ops::Index;
use std::rc::Rc;
use view::{RcView, ViewContext};

use control_object::ControlObject;

pub trait ChildrenSource {
    fn len(&self) -> usize;
    fn index(&self, index: usize) -> Rc<RefCell<dyn ControlObject>>;
}

pub struct ChildrenSourceIterator<'a> {
    source: &'a ChildrenSource,
    pos: usize,
    len: usize,
}

impl<'a> Iterator for ChildrenSourceIterator<'a> {
    type Item = Rc<RefCell<dyn ControlObject>>;

    fn next(&mut self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if self.pos < self.len {
            self.pos += 1;
            Some(self.source.index(self.pos - 1))
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for ChildrenSourceIterator<'a> {
    fn next_back(&mut self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if self.len > self.pos {
            self.len -= 1;
            Some(self.source.index(self.len))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a ChildrenSource {
    type Item = Rc<RefCell<dyn ControlObject>>;
    type IntoIter = ChildrenSourceIterator<'a>;

    fn into_iter(self) -> ChildrenSourceIterator<'a> {
        ChildrenSourceIterator {
            source: self,
            pos: 0,
            len: self.len(),
        }
    }
}

///
/// StaticChildrenSource.
///
pub struct StaticChildrenSource {
    children: Vec<Rc<RefCell<dyn ControlObject>>>,
}

impl StaticChildrenSource {
    pub fn new(children: Vec<Rc<RefCell<dyn ControlObject>>>) -> Self {
        StaticChildrenSource { children }
    }
}

impl ChildrenSource for StaticChildrenSource {
    fn len(&self) -> usize {
        self.children.len()
    }

    fn index(&self, index: usize) -> Rc<RefCell<dyn ControlObject>> {
        self.children.index(index).clone()
    }
}

///
/// DynamicChildrenSource.
///
pub struct DynamicChildrenSource {
    children: Rc<RefCell<Vec<Rc<RefCell<dyn ControlObject>>>>>,
    changed_event_subscription: EventSubscription,
}

impl DynamicChildrenSource {
    pub fn new<T>(children: &ObservableVec<Rc<RefCell<T>>>) -> Self
    where
        T: RcView,
    {
        let children_vec = children
            .into_iter()
            .map(|vm| RcView::to_view(&vm, ViewContext::empty()))
            .collect();

        let children_rc = Rc::new(RefCell::new(children_vec));

        let children_rc_clone = children_rc.clone();
        let event_subscription =
            children
                .get_changed_event()
                .borrow_mut()
                .subscribe(move |changed_args| match changed_args {
                    ChangedEventArgs::Insert { index, value } => {
                        let mut vec: RefMut<'_, Vec<Rc<RefCell<dyn ControlObject>>>> =
                            children_rc_clone.borrow_mut();
                        vec.insert(index, RcView::to_view(&value, ViewContext::empty()));
                    }
                    ChangedEventArgs::Remove { index, value: _value } => {
                        let mut vec: RefMut<'_, Vec<Rc<RefCell<dyn ControlObject>>>> =
                            children_rc_clone.borrow_mut();
                        vec.remove(index);
                    }
                });

        DynamicChildrenSource {
            children: children_rc,
            changed_event_subscription: event_subscription,
        }
    }
}

impl ChildrenSource for DynamicChildrenSource {
    fn len(&self) -> usize {
        self.children.borrow().len()
    }

    fn index(&self, index: usize) -> Rc<RefCell<dyn ControlObject>> {
        self.children.borrow().index(index).clone()
    }
}

///
/// AggregatedChildrenSource.
///
pub struct AggregatedChildrenSource {
    sources: Vec<Box<dyn ChildrenSource>>,
}

impl AggregatedChildrenSource {
    pub fn new(sources: Vec<Box<dyn ChildrenSource>>) -> Self {
        AggregatedChildrenSource { sources }
    }
}

impl ChildrenSource for AggregatedChildrenSource {
    fn len(&self) -> usize {
        self.sources.iter().map(|s| s.len()).sum()
    }

    fn index(&self, index: usize) -> Rc<RefCell<dyn ControlObject>> {
        let mut new_index = index;
        for source in &self.sources {
            let len = source.len();
            if new_index < len {
                return source.index(new_index);
            } else {
                new_index -= len;
            }
        }

        panic!(format!(
            "index out of bounds: the len is {} but the index is {}",
            self.len(),
            index
        ))
    }
}
