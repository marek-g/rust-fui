use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

use children_source::*;
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

///
/// Used to convert controls to views.
/// Controls can be consumed during conversion.
///
pub trait View {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>>;
}

///
/// Used to convert view models to views.
/// Data from view models can be only borrowed (not consumed) during conversion.
pub trait RcView {
    fn to_view(view_model: &Rc<RefCell<Self>>, context: ViewContext) -> Rc<RefCell<ControlObject>>;
}
