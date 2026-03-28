use crate::view::ViewContext;
use std::cell::{Cell, Ref, RefCell};
use std::rc::{Rc, Weak};

use crate::{control::*, TypeMap, TypeMapKey};
use crate::{observable::*, spawn_local_and_forget, Children, Rect, Services};

pub struct ControlContext {
    self_weak: RefCell<Option<Weak<dyn ControlObject>>>,
    parent: RefCell<Option<Weak<dyn ControlObject>>>,
    is_attached: Cell<bool>,
    children: Children,

    children_collection_changed_event_subscription: RefCell<Option<Subscription>>,
    dirty_event_subscriptions: RefCell<Vec<Subscription>>,

    attached_values: RefCell<TypeMap>,
    inherited_values_cache: RefCell<TypeMap>,

    services: RefCell<Option<Services>>,

    rect: Cell<Rect>,

    is_dirty: Cell<bool>,
}

impl ControlContext {
    pub fn new(view_context: ViewContext) -> Self {
        ControlContext {
            self_weak: RefCell::new(None),
            parent: RefCell::new(None),
            is_attached: Cell::new(false),
            children: view_context.children,
            children_collection_changed_event_subscription: RefCell::new(None),
            dirty_event_subscriptions: RefCell::new(Vec::new()),
            attached_values: RefCell::new(view_context.attached_values),
            inherited_values_cache: RefCell::new(TypeMap::new()),
            services: RefCell::new(None),
            rect: Cell::new(Rect::empty()),
            is_dirty: Cell::new(true),
        }
    }

    pub fn get_self_weak(&self) -> Weak<dyn ControlObject> {
        self.self_weak.borrow().clone().unwrap()
    }

    pub fn get_self_rc(&self) -> Rc<dyn ControlObject> {
        self.self_weak.borrow().as_ref().unwrap().upgrade().unwrap()
    }

    pub fn set_self(&self, self_weak: Weak<dyn ControlObject>) {
        *self.self_weak.borrow_mut() = Some(self_weak);
    }

    pub fn get_parent(&self) -> Option<Rc<dyn ControlObject>> {
        if let Some(ref test) = *self.parent.borrow() {
            test.upgrade()
        } else {
            None
        }
    }

    pub fn set_parent(&self, parent_rc: &Rc<dyn ControlObject>) {
        *self.parent.borrow_mut() = Some(Rc::downgrade(parent_rc));
        self.inherited_values_cache.borrow_mut().clear();

        if parent_rc.get_context().is_attached.get() {
            self.attach_tree();
        }
    }

    pub fn get_children(&self) -> &Children {
        &self.children
    }

    pub fn set_children_collection_changed_event_subscription(&self, s: Option<Subscription>) {
        *self
            .children_collection_changed_event_subscription
            .borrow_mut() = s;
    }

    pub fn get_services(&self) -> Option<Services> {
        self.services.borrow().clone()
    }

    pub fn set_services(&self, services: Option<Services>) {
        for child in self.children.into_iter() {
            child.get_context().set_services(services.clone());
        }
        *self.services.borrow_mut() = services;
    }

    // control is attached to the window
    pub fn attach_tree(&self) {
        self.is_attached.set(true);
        self.get_self_rc().parent_attached();
        for child in self.children.into_iter() {
            child.get_context().is_attached.set(true);
            child.get_context().attach_tree();
        }
    }

    pub fn get_attached_value<K: TypeMapKey + 'static>(&self) -> Option<Ref<'_, K::Value>> {
        Ref::filter_map(self.attached_values.borrow(), |map| map.get::<K>()).ok()
    }

    pub fn get_inherited_value<K: TypeMapKey + 'static>(&self) -> Option<K::Value>
    where
        K::Value: Clone,
    {
        // Check cache first
        if let Some(cached) = self.inherited_values_cache.borrow().get::<K>() {
            return Some(cached.clone());
        }

        // Check local attached values (don't cache these)
        if let Some(val) = self.get_attached_value::<K>() {
            return Some(val.clone());
        }

        // Traverse parents to find inherited value
        let mut current_parent = self.get_parent();
        let mut found_in_parent = false;
        let mut result = None;

        while let Some(parent) = current_parent {
            let context = parent.get_context();
            if let Some(val) = context.get_attached_value::<K>() {
                result = Some(val.clone());
                found_in_parent = true;
                break;
            }
            current_parent = context.get_parent();
        }

        // Cache the value only if we found it in a parent (not locally)
        if found_in_parent {
            if let Some(ref val) = result {
                self.inherited_values_cache
                    .borrow_mut()
                    .insert::<K>(val.clone());
            }
        }

        result
    }

    pub fn set_attached_values(&self, attached_values: TypeMap) {
        *self.attached_values.borrow_mut() = attached_values;
    }

    pub fn get_rect(&self) -> Rect {
        self.rect.get()
    }

    pub fn set_rect(&self, rect: Rect) {
        self.rect.set(rect);
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty.get()
    }

    pub fn set_is_dirty(&self, is_dirty: bool) {
        let is_change = self.is_dirty.get() != is_dirty;
        self.is_dirty.set(is_dirty);

        if is_dirty {
            match self.get_parent() {
                Some(ref parent) => parent.get_context().set_is_dirty(is_dirty),
                _ => {
                    // this is a root control
                    if is_change {
                        // post window repaint
                        if let Some(services) = self.services.borrow().clone() {
                            spawn_local_and_forget(async move {
                                services.get_window_service().map(|s| s.repaint());
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn dirty_watch_property<T>(&self, property: &Property<T>)
    where
        T: 'static + Clone + PartialEq,
    {
        let self_weak = self.self_weak.borrow().clone().unwrap();
        self.dirty_event_subscriptions
            .borrow_mut()
            .push(property.on_changed(move |_| {
                self_weak.upgrade().map(|control| {
                    control.get_context().set_is_dirty(true);
                });
            }));
    }

    pub fn dirty_watch_attached_properties(&self) {
        let attached_values = self.attached_values.borrow();
        if let Some(visible) = attached_values.get::<Visible>() {
            let self_weak = self.self_weak.borrow().clone().unwrap();
            let mut subs = self.dirty_event_subscriptions.borrow_mut();
            subs.push(visible.on_changed(move |_| {
                self_weak.upgrade().map(|control| {
                    control.get_context().set_is_dirty(true);
                });
            }));
        }
    }
}
