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

pub struct RootView {
    pub view_data: ViewData,
}

impl RootView {
    pub fn new(view_data: ViewData) -> Self {
        RootView {
            view_data: view_data,
        }
    }
}
