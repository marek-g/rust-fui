use std::cell::RefCell;
use std::rc::{Rc, Weak};
use typemap::TypeMap;

use crate::children_source::*;
use crate::control::*;
use crate::observable::*;

pub struct ControlContext {
    pub attached_values: TypeMap,
    pub children: Box<dyn ChildrenSource>,

    pub parent: Option<Weak<RefCell<dyn ControlObject>>>,
    pub is_dirty: bool,
    pub children_collection_changed_event_subscription: Option<EventSubscription>,
}

impl ControlContext {
    pub fn get_attached_values(&self) -> &TypeMap {
        &self.attached_values
    }

    pub fn get_children(&self) -> &Box<dyn ChildrenSource> {
        &self.children
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if let Some(ref test) = self.parent {
            test.upgrade()
        } else {
            None
        }
    }

    pub fn set_parent(&mut self, parent: Weak<RefCell<dyn ControlObject>>) {
        self.parent = Some(parent);
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn set_is_dirty(&mut self, is_dirty: bool) {
        self.is_dirty = is_dirty;
        if is_dirty {
            if let Some(ref parent) = self.get_parent() {
                parent.borrow_mut().set_is_dirty(is_dirty)
            }
        }
    }
}
