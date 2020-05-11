use crate::view::ViewContext;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use typemap::TypeMap;

use crate::control::*;
use crate::observable::*;

pub struct ControlContext {
    parent: Option<Weak<RefCell<dyn ControlObject>>>,
    children: Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>,
    children_collection_changed_event_subscription: Option<EventSubscription>,

    attached_values: TypeMap,

    is_dirty: bool,
}

impl ControlContext {
    pub fn new(view_context: ViewContext) -> Self {
        ControlContext {
            attached_values: view_context.attached_values,
            children: view_context.children,
            parent: None,
            is_dirty: true,
            children_collection_changed_event_subscription: None,
        }
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

    pub fn get_children(&self) -> &Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>> {
        &self.children
    }

    pub fn set_children_collection_changed_event_subscription(
        &mut self,
        s: Option<EventSubscription>,
    ) {
        self.children_collection_changed_event_subscription = s;
    }

    pub fn get_attached_values(&self) -> &TypeMap {
        &self.attached_values
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn set_is_dirty(&mut self, is_dirty: bool) {
        self.is_dirty = is_dirty;
        if is_dirty {
            if let Some(ref parent) = self.get_parent() {
                parent.borrow_mut().get_context_mut().set_is_dirty(is_dirty)
            }
        }
    }
}
