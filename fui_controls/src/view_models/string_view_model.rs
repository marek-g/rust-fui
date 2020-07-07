use crate::Text;
use fui::{ControlObject, ObservableCollection, ViewContext, ViewModel, ViewModelObject};
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

///
/// This view model is useful to use with many controls
/// to represent simple static text.
/// For example as an text item for DropDown's items collection.
///
pub struct StringViewModel {
    pub text: String,
}

impl StringViewModel {
    pub fn new<T: Into<String>>(text: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(StringViewModel { text: text.into() }))
    }
}

impl ViewModel for StringViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            Text { text: &*view_model.borrow().text }
        }
    }
}
