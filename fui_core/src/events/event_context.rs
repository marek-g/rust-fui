use crate::{ControlEvent, ControlObject};
use std::rc::Rc;

pub trait EventContext {
    fn get_captured_control(&self) -> Option<Rc<dyn ControlObject>>;
    fn set_captured_control(&mut self, control: Option<Rc<dyn ControlObject>>);

    fn get_focused_control(&self) -> Option<Rc<dyn ControlObject>>;
    fn set_focused_control(&mut self, control: Option<Rc<dyn ControlObject>>);

    fn queue_event(&mut self, control: Option<Rc<dyn ControlObject>>, event: ControlEvent);
}
