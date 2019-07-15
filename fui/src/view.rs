use std::cell::RefCell;
use std::rc::Rc;

use control_object::ControlObject;

pub struct ViewContext {
    pub children: Vec<Rc<RefCell<ControlObject>>>,
}

pub trait View {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>>;
}
