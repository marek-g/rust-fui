use windowing_api::{CursorShape, Edge};

use crate::ControlObject;
use std::rc::Rc;

pub trait WindowService {
    fn add_layer(&self, control: Rc<dyn ControlObject>);
    fn remove_layer(&self, control: &Rc<dyn ControlObject>);
    fn repaint(&self);

    fn set_cursor(&self, cursor_shape: CursorShape);
    fn start_system_move(&self);
    fn start_system_resize(&self, edges: Edge);
}
