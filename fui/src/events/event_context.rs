use std::{cell::RefCell, rc::Rc};
use crate::{ControlEvent, ControlObject};

pub trait EventContext {
    fn get_hovered_control(&self) -> Option<Rc<RefCell<dyn ControlObject>>>;
    fn set_hovered_control(&mut self,control: Option<Rc<RefCell<dyn ControlObject>>>);

    fn get_captured_control(&self) -> Option<Rc<RefCell<dyn ControlObject>>>;
    fn set_captured_control(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>);

    fn get_focused_control(&self) -> Option<Rc<RefCell<dyn ControlObject>>>;
    fn set_focused_control(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>);

    fn queue_event(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>, event: ControlEvent);
}
