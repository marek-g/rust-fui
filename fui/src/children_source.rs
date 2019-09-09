use std::cell::RefCell;
use std::ops::Index;
use std::rc::Rc;
use view::{RcView, ViewContext};
use observable::ObservableVec;

use control_object::ControlObject;

pub trait ChildrenSource {
    fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>>;
    fn len(&self) -> usize;
    fn index<'a>(&'a self, index: usize) -> &'a Rc<RefCell<dyn ControlObject>>;
}

impl<'a> IntoIterator for &'a ChildrenSource {
    type Item = &'a Rc<RefCell<dyn ControlObject>>;
    type IntoIter = ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>>;

    fn into_iter(self) -> ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>> {
        self.iter()
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
    fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>> {
        self.children.iter()
    }

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
    fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>> {
        self.children.iter()
    }

    fn len(&self) -> usize {
        self.children.len()
    }

    fn index<'a>(&'a self, index: usize) -> &'a Rc<RefCell<dyn ControlObject>> {
        self.children.index(index)
    }
}
