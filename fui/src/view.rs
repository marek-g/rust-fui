use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

use crate::{ObservableCollection, control::ControlObject, Style};

pub struct ViewContext {
    pub attached_values: TypeMap,
    pub children: Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>,
}

impl ViewContext {
    pub fn empty() -> ViewContext {
        ViewContext {
            attached_values: TypeMap::new(),
            children: Box::new(Vec::new()),
        }
    }
}

///
/// Used to convert controls to views.
/// Controls can be consumed during conversion.
///
pub trait Control {
    fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>>;
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
