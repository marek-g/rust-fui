use crate::view::ViewContext;
use std::cell::{Cell, Ref, RefCell};
use std::rc::{Rc, Weak};

use crate::{control::*, TypeMap, TypeMapKey};
use crate::{observable::*, spawn_local_and_forget, Children, Rect, Services};

pub struct ControlContext {
    self_weak: RefCell<Option<Weak<RefCell<dyn ControlObject>>>>,
    parent: RefCell<Option<Weak<RefCell<dyn ControlObject>>>>,
    children: Children,

    children_collection_changed_event_subscription: RefCell<Option<Subscription>>,
    dirty_event_subscriptions: RefCell<Vec<Subscription>>,

    attached_values: RefCell<TypeMap>,

    services: RefCell<Option<Services>>,

    rect: Cell<Rect>,

    is_dirty: Cell<bool>,
}

impl ControlContext {
    pub fn new(view_context: ViewContext) -> Self {
        ControlContext {
            self_weak: RefCell::new(None),
            parent: RefCell::new(None),
            children: view_context.children,
            children_collection_changed_event_subscription: RefCell::new(None),
            dirty_event_subscriptions: RefCell::new(Vec::new()),
            attached_values: RefCell::new(view_context.attached_values),
            services: RefCell::new(None),
            rect: Cell::new(Rect::empty()),
            is_dirty: Cell::new(true),
        }
    }

    pub fn get_self_weak(&self) -> Weak<RefCell<dyn ControlObject>> {
        self.self_weak.borrow().clone().unwrap()
    }

    pub fn get_self_rc(&self) -> Rc<RefCell<dyn ControlObject>> {
        self.self_weak.borrow().as_ref().unwrap().upgrade().unwrap()
    }

    pub fn set_self(&self, self_weak: Weak<RefCell<dyn ControlObject>>) {
        *self.self_weak.borrow_mut() = Some(self_weak);
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if let Some(ref test) = *self.parent.borrow() {
            test.upgrade()
        } else {
            None
        }
    }

    pub fn set_parent(&self, parent_rc: &Rc<RefCell<dyn ControlObject>>) {
        *self.parent.borrow_mut() = Some(Rc::downgrade(parent_rc));
    }

    pub fn get_children(&self) -> &Children {
        &self.children
    }

    pub fn set_children_collection_changed_event_subscription(&self, s: Option<Subscription>) {
        *self.children_collection_changed_event_subscription.borrow_mut() = s;
    }

    pub fn get_attached_value<K: TypeMapKey + 'static>(&self) -> Option<Ref<'_, K::Value>> {
        Ref::filter_map(self.attached_values.borrow(), |map| map.get::<K>()).ok()
    }

    pub fn set_attached_values(&self, attached_values: TypeMap) {
        *self.attached_values.borrow_mut() = attached_values;
    }

    pub fn get_services(&self) -> Option<Services> {
        self.services.borrow().clone()
    }

    pub fn set_services(&self, services: Option<Services>) {
        for child in self.children.into_iter() {
            child
                .borrow()
                .get_context()
                .set_services(services.clone());
        }
        *self.services.borrow_mut() = services;
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
                Some(ref parent) => parent.borrow().get_context().set_is_dirty(is_dirty),
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
                    control.borrow().get_context().set_is_dirty(true);
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
                    control.borrow().get_context().set_is_dirty(true);
                });
            }));
        }
    }
}
