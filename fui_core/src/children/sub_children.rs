use crate::{ControlObject, ObservableCollection};
use std::cell::RefCell;
use std::rc::Rc;

pub enum SubChildren {
    SingleStatic(Rc<RefCell<dyn ControlObject>>),
    SingleDynamic(Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>),
    MultipleStatic(Vec<Rc<RefCell<dyn ControlObject>>>),
}

impl SubChildren {
    pub fn len(&self) -> usize {
        match self {
            SubChildren::SingleStatic(_) => 1,
            SubChildren::SingleDynamic(x) => x.len(),
            SubChildren::MultipleStatic(x) => x.len(),
        }
    }

    pub fn get(&self, index: usize) -> Option<Rc<RefCell<dyn ControlObject>>> {
        match self {
            SubChildren::SingleStatic(x) => {
                if index == 0 {
                    Some(x.clone())
                } else {
                    None
                }
            }
            SubChildren::SingleDynamic(x) => x.get(index),
            SubChildren::MultipleStatic(x) => x.get(index),
        }
    }
}
