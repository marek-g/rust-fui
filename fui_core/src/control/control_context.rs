use crate::view::ViewContext;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use typemap::TypeMap;

use crate::control::*;
use crate::{observable::*, spawn_local_and_forget, Children, Rect, Services};

pub struct ControlContext {
    self_weak: Option<Weak<RefCell<dyn ControlObject>>>,
    parent: Option<Weak<RefCell<dyn ControlObject>>>,
    children: Children,

    children_collection_changed_event_subscription: Option<Subscription>,
    dirty_event_subscriptions: Vec<Subscription>,

    attached_values: TypeMap,

    services: Option<Weak<RefCell<Services>>>,

    rect: Rect,

    is_dirty: bool,
}

impl ControlContext {
    pub fn new(view_context: ViewContext) -> Self {
        ControlContext {
            self_weak: None,
            parent: None,
            children: view_context.children,
            children_collection_changed_event_subscription: None,
            dirty_event_subscriptions: Vec::new(),
            attached_values: view_context.attached_values,
            services: None,
            rect: Rect::empty(),
            is_dirty: true,
        }
    }

    pub fn get_self_weak(&self) -> Weak<RefCell<dyn ControlObject>> {
        self.self_weak.clone().unwrap()
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
    }

    pub fn get_children(&self) -> &Children {
        &self.children
    }

    pub fn set_children_collection_changed_event_subscription(&mut self, s: Option<Subscription>) {
        self.children_collection_changed_event_subscription = s;
    }

    pub fn get_attached_values(&self) -> &TypeMap {
        &self.attached_values
    }

    pub fn set_attached_values(&mut self, attached_values: TypeMap) {
        self.attached_values = attached_values;
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
            child
                .borrow_mut()
                .get_context_mut()
                .set_services(services.clone());
        }
        self.services = services;
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn set_is_dirty(&mut self, is_dirty: bool) {
        let is_change = self.is_dirty != is_dirty;
        self.is_dirty = is_dirty;

        if is_dirty {
            if let Some(ref parent) = self.get_parent() {
                parent.borrow_mut().get_context_mut().set_is_dirty(is_dirty)
            } else {
                // this is a root control
                if is_change {
                    // post window repaint
                    // (cannot call it directly because services can be already borrowed)
                    if let Some(services) = self.services.clone() {
                        spawn_local_and_forget(async move {
                            if let Some(services) = services.upgrade() {
                                services
                                    .borrow_mut()
                                    .get_window_service()
                                    .map(|s| s.borrow_mut().repaint());
                            }
                        });
                    }
                }
            }
        }
    }

    pub fn dirty_watch_property<T>(&mut self, property: &Property<T>)
    where
        T: 'static + Clone + PartialEq,
    {
        let self_weak = self.self_weak.clone().unwrap();
        self.dirty_event_subscriptions
            .push(property.on_changed(move |_| {
                self_weak.upgrade().map(|control| {
                    control.borrow_mut().get_context_mut().set_is_dirty(true);
                });
            }));
    }

    pub fn dirty_watch_attached_properties(&mut self) {
        if let Some(visible) = self.attached_values.get::<Visible>() {
            let self_weak = self.self_weak.clone().unwrap();
            self.dirty_event_subscriptions
                .push(visible.on_changed(move |_| {
                    self_weak.upgrade().map(|control| {
                        control.borrow_mut().get_context_mut().set_is_dirty(true);
                    });
                }));
        }
    }
}
