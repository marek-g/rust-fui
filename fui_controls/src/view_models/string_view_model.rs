use crate::Text;
use fui_core::{Children, ControlObject, ViewContext, ViewModel};
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

///
/// This view model is useful to use with many controls
/// to represent simple static text.
/// For example as an text item for DropDown's items collection.
///
#[derive(PartialEq)]
pub struct StringViewModel {
    pub text: String,
}

impl StringViewModel {
    pub fn new<T: Into<String>>(text: T) -> Rc<Self> {
        Rc::new(StringViewModel { text: text.into() })
    }
}

impl ViewModel for StringViewModel {
    fn create_view(self: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            Text { text: &*self.text }
        }
    }
}
