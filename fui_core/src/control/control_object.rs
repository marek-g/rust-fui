use crate::control::control_behavior::ControlBehavior;
use crate::{
    control::control_context::ControlContext, EventSubscription, JoinHandle, Point, Property,
};
use std::rc::Weak;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

pub trait ControlObject: ControlBehavior {
    fn get_context(&self) -> &ControlContext;
    fn get_context_mut(&mut self) -> &mut ControlContext;

    ///
    /// Returns all the child controls including this one
    /// that are located within a specified `point` of the window.
    /// This also includes controls placed behind hit controls.
    ///
    /// The order is from the bottom to the top.
    ///
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

    ///
    /// Returns the `hit` control and all its parent controls
    /// up to and including this one.
    ///
    fn get_hit_path(&self, point: Point) -> Vec<Weak<RefCell<dyn ControlObject>>> {
        let mut res = Vec::new();
        let mut hit_control = self.hit_test(point);
        while let Some(control) = &hit_control {
            res.push(Rc::downgrade(control));

            hit_control = if Rc::ptr_eq(&control, &self.get_context().get_self_rc()) {
                None
            } else {
                control.borrow().get_context().get_parent()
            }
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
    fn dirty_watching(&self, control: &Rc<RefCell<dyn ControlObject>>) -> JoinHandle<()>;
}

impl<T> PropertyDirtyExtension for Property<T>
where
    T: 'static + Clone + PartialEq,
{
    fn dirty_watching(&self, control: &Rc<RefCell<dyn ControlObject>>) -> JoinHandle<()> {
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
