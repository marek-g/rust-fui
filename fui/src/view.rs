use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

use crate::children_source::*;
use crate::control::ControlObject;

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

//pub type View = 

///
/// Used to convert controls to views.
/// Controls can be consumed during conversion.
///
pub trait Control {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>>;
}

///
/// Used to convert view models to views.
/// Data from view models can be only borrowed (not consumed) during conversion.
///
pub trait ViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
    ) -> Rc<RefCell<dyn ControlObject>>;
}
