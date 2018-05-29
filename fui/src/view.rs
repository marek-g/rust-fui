use std::cell::RefCell;
use std::rc::Rc;
use control::ControlObject;

pub struct ViewData {
    pub root_control: Box<ControlObject>,
}

pub trait View {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> ViewData;
}
