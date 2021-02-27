use crate::ControlObject;
use std::{cell::RefCell, rc::Rc};

pub trait WindowService {
    fn add_layer(&mut self, control: Rc<RefCell<dyn ControlObject>>);
    fn remove_layer(&mut self, control: &Rc<RefCell<dyn ControlObject>>);
}
