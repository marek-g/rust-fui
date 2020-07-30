use crate::control::control_behavior::ControlBehavior;
use crate::{EventSubscription, control::control_context::ControlContext, Property};
use std::{cell::{RefMut, RefCell}, rc::Rc};

pub trait ControlObject: ControlBehavior {
    fn get_context(&self) -> &ControlContext;
    fn get_context_mut(&mut self) -> &mut ControlContext;
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
