use crate::control::control_behavior::ControlBehavior;
use crate::{control::control_context::ControlContext, EventSubscription, Point, Property};
use std::rc::Weak;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

pub trait ControlObject: ControlBehavior {
    fn get_context(&self) -> &ControlContext;
    fn get_context_mut(&mut self) -> &mut ControlContext;

    /// Returns all the child controls including this one
    /// that are located within a specified `point` of the window.
    ///
    /// The order is from the bottom to the top.
    fn get_controls_at_point(&self, point: Point) -> Vec<Weak<RefCell<dyn ControlObject>>> {
        let rect = self.get_rect();
        let mut res = Vec::new();
        if point.is_inside(&rect) {
            let children = self.get_context().get_children();
            for child in children {
                res.append(&mut child.borrow().get_controls_at_point(point));
            }
            res.push(self.get_context().get_self_weak())
        }
        res
    }
}

///
/// PartialEq implementation allows using ControlObject with Property.
///
impl PartialEq for dyn ControlObject {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self, &other)
    }
}

pub trait PropertyDirtyExtension {
    fn dirty_watching(&self, control: &Rc<RefCell<dyn ControlObject>>) -> EventSubscription;
}

impl<T> PropertyDirtyExtension for Property<T>
where
    T: 'static + Clone + PartialEq,
{
    fn dirty_watching(&self, control: &Rc<RefCell<dyn ControlObject>>) -> EventSubscription {
        let weak_control = Rc::downgrade(control);
        self.on_changed(move |_| {
            weak_control.upgrade().map(|control| {
                (control.borrow_mut() as RefMut<dyn ControlObject>)
                    .get_context_mut()
                    .set_is_dirty(true)
            });
        })
    }
}
