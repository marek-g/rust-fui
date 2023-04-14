use fui_system_core::{CursorShape, Edge};

use crate::ControlObject;
use std::{cell::RefCell, rc::Rc};

pub trait WindowService {
    fn add_layer(&mut self, control: Rc<RefCell<dyn ControlObject>>);
    fn remove_layer(&mut self, control: &Rc<RefCell<dyn ControlObject>>);
    fn repaint(&mut self);

    fn set_cursor(&mut self, cursor_shape: CursorShape);
    fn start_system_move(&mut self);
    fn start_system_resize(&mut self, edges: Edge);
}
