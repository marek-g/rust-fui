use std::cell::RefCell;
use std::rc::Rc;
use control::ControlObject;
use Binding;

pub struct ViewData {
    pub root_control: Rc<RefCell<ControlObject>>,
    pub bindings: Vec<Box<Binding>>
}

pub trait View {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> ViewData;
}
