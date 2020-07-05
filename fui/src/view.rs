use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

use crate::{control::ControlObject, ObservableCollection};

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
//pub trait Control: Sized {
//    fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<StyledControl<Self>>>;
//}
//
// Instead of using this trait we expect control structs to just implement this method (by convention).
// It will be called from ui!() macro.
// The reason why it is not a trait is that sometimes composite controls may not want
// to return StyledControl<Self> but the root control of the composition that may be
// for example StyledControl<Grid> or just dyn ControlObject.
// Also we cannot always return dyn ControlObject, because sometimes we may want to create controls
// with ui!() macro and later have access to its type, for example
// in a cases like creating radio buttons and passing their references to the radio button controller.
//

///
/// Used to convert view models to views.
/// Data from view models can be only borrowed (not consumed) during conversion.
///
pub trait ViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>>;
}

///
/// Object safe version of ViewModel's trait.
///
pub trait ViewModelObject {
    fn create_view(&self) -> Rc<RefCell<dyn ControlObject>>;
}

impl<T: ViewModel> ViewModelObject for Rc<RefCell<T>> {
    fn create_view(&self) -> Rc<RefCell<dyn ControlObject>> {
        ViewModel::create_view(self)
    }
}
