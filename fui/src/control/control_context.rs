use crate::view::ViewContext;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use typemap::TypeMap;

use crate::control::*;
use crate::{Services, observable::*};

pub struct ControlContext {
    self_weak: Option<Weak<RefCell<dyn ControlObject>>>,
    parent: Option<Weak<RefCell<dyn ControlObject>>>,
    children: Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>,
    children_collection_changed_event_subscription: Option<EventSubscription>,

    attached_values: TypeMap,

    services: Option<Weak<RefCell<Services>>>,

    is_dirty: bool,
}

impl ControlContext {
    pub fn new(view_context: ViewContext) -> Self {
        ControlContext {
            self_weak: None,
            parent: None,
            children: view_context.children,
            children_collection_changed_event_subscription: None,
            attached_values: view_context.attached_values,
            services: None,
            is_dirty: true,
        }
    }

    pub fn get_self_rc(&self) -> Rc<RefCell<dyn ControlObject>> {
        self.self_weak.as_ref().unwrap().upgrade().unwrap()
    }

    pub fn set_self(&mut self, self_weak: Weak<RefCell<dyn ControlObject>>) {
        self.self_weak = Some(self_weak);
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if let Some(ref test) = self.parent {
            test.upgrade()
        } else {
            None
        }
    }

    pub fn set_parent(&mut self, parent_rc: &Rc<RefCell<dyn ControlObject>>) {
        self.parent = Some(Rc::downgrade(parent_rc));
        let services = parent_rc.borrow_mut().get_context().get_services();
        self.set_services(services);
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

    ///
    /// Available only when control is added to the window.
    /// Not yet set during control setup().
    ///
    pub fn get_services(&self) -> Option<Weak<RefCell<Services>>> {
        self.services.clone()
    }

    pub fn set_services(&mut self, services: Option<Weak<RefCell<Services>>>) {
        for child in self.children.into_iter() {
            child.borrow_mut().get_context_mut().set_services(services.clone());
        }
        self.services = services;
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
