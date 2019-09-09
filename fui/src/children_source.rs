use observable::ObservableVec;
use std::cell::RefCell;
use std::ops::Index;
use std::rc::Rc;
use view::{RcView, ViewContext};

use control_object::ControlObject;

pub trait ChildrenSource {
    fn len(&self) -> usize;
    fn index<'a>(&'a self, index: usize) -> &'a Rc<RefCell<dyn ControlObject>>;
}

pub struct ChildrenSourceIterator<'a> {
    source: &'a ChildrenSource,
    pos: usize,
    len: usize,
}

impl<'a> Iterator for ChildrenSourceIterator<'a> {
    type Item = &'a Rc<RefCell<dyn ControlObject>>;

    fn next(&mut self) -> Option<&'a Rc<RefCell<dyn ControlObject>>> {
        if self.pos < self.len {
            self.pos += 1;
            Some(self.source.index(self.pos - 1))
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for ChildrenSourceIterator<'a> {
    fn next_back(&mut self) -> Option<&'a Rc<RefCell<dyn ControlObject>>> {
        if self.len > self.pos {
            self.len -= 1;
            Some(self.source.index(self.len))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a ChildrenSource {
    type Item = &'a Rc<RefCell<dyn ControlObject>>;
    type IntoIter = ChildrenSourceIterator<'a>;

    fn into_iter(self) -> ChildrenSourceIterator<'a> {
        ChildrenSourceIterator {
            source: self,
            pos: 0,
            len: self.len(),
        }
    }
}

impl Index<usize> for ChildrenSource {
    type Output = Rc<RefCell<dyn ControlObject>>;

    fn index(&self, index: usize) -> &Self::Output {
        self.index(index)
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

    fn index<'a>(&'a self, index: usize) -> &'a Rc<RefCell<dyn ControlObject>> {
        self.children.index(index)
    }
}

///
/// DynamicChildrenSource.
///
pub struct DynamicChildrenSource {
    children: Vec<Rc<RefCell<dyn ControlObject>>>,
}

impl DynamicChildrenSource {
    pub fn new<T>(children: &ObservableVec<T>) -> Self
    where
        T: RcView,
    {
        DynamicChildrenSource {
            children: children
                .into_iter()
                .map(|vm| RcView::to_view(&vm, ViewContext::empty()))
                .collect(),
        }
    }
}

impl ChildrenSource for DynamicChildrenSource {
    fn len(&self) -> usize {
        self.children.len()
    }

    fn index<'a>(&'a self, index: usize) -> &'a Rc<RefCell<dyn ControlObject>> {
        self.children.index(index)
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

    fn index<'a>(&'a self, index: usize) -> &'a Rc<RefCell<dyn ControlObject>> {
        let mut new_index = index;
        for source in &self.sources {
            let len = source.len();
            if new_index < len {
                return source.index(new_index);
            } else {
                new_index -= len;
            }
        }

        panic!(format!("index out of bounds: the len is {} but the index is {}",
            self.len(), index))
    }
}
