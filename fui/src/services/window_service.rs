use std::{cell::RefCell, rc::Rc};
use crate::ControlObject;

pub trait WindowService {
    fn add_new_layer(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>);
    fn remove_layer(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>);
}
