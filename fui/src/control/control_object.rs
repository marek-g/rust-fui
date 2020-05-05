use crate::control::control_behiavor::ControlBehavior;
use crate::control::control_context::ControlContext;

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
