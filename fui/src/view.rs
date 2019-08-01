use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

use children_collection::*;
use control_object::ControlObject;

pub struct ViewContext {
    pub attached_values: TypeMap,
    pub children: Box<dyn ChildrenSource>,
}

impl ViewContext {
    pub fn empty() -> ViewContext {
        ViewContext {
            attached_values: TypeMap::new(),
            children: Box::new(StaticChildrenSource::new(Vec::new())),
        }
    }
}

pub trait View {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>>;
}
