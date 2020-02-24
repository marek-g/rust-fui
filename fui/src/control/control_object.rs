use crate::control::control_behiavor::ControlBehavior;
use crate::control::control_context::ControlContext;

pub trait ControlObject {
    fn get_context(&self) -> &ControlContext;
    fn get_context_mut(&mut self) -> &mut ControlContext;

    fn get_behavior(&self) -> &dyn ControlBehavior;
    fn get_behavior_mut(&mut self) -> &mut dyn ControlBehavior;
}
