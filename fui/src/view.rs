use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

use control_object::ControlObject;

pub struct ViewContext {
    pub attached_values: TypeMap,
    pub children: Vec<Rc<RefCell<ControlObject>>>,
}

impl ViewContext {
    pub fn empty() -> ViewContext {
        ViewContext {
            attached_values: TypeMap::new(),
            children: Vec::new(),
        }
    }
}

pub trait View {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>>;
}
