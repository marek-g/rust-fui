use crate::control::control_behiavor::ControlBehavior;
use crate::control::control_context::ControlContext;

pub trait ControlObject: ControlBehavior {
    fn get_context(&self) -> &ControlContext;
    fn get_context_mut(&mut self) -> &mut ControlContext;
}
