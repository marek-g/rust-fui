use crate::{Children, ControlObject};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ChildrenIterator<'a> {
    source: &'a Children,
    pos: usize,
    len: usize,
}

impl<'a> Iterator for ChildrenIterator<'a> {
    type Item = Rc<RefCell<dyn ControlObject>>;

    fn next(&mut self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if self.pos < self.len {
            self.pos += 1;
            self.source.get(self.pos - 1)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for ChildrenIterator<'a> {
    fn next_back(&mut self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if self.len > self.pos {
            self.len -= 1;
            self.source.get(self.len)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Children {
    type Item = Rc<RefCell<dyn ControlObject>>;
    type IntoIter = ChildrenIterator<'a>;

    fn into_iter(self) -> ChildrenIterator<'a> {
        ChildrenIterator {
            source: self,
            pos: 0,
            len: self.len(),
        }
    }
}
