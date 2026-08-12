use std::rc::Rc;

use crate::{Children, Style, TypeMap, control::ControlObject};

pub struct ViewContext {
    pub attached_values: TypeMap,
    pub children: Children,
}

impl ViewContext {
    pub fn empty() -> ViewContext {
        ViewContext {
            attached_values: TypeMap::new(),
            children: Children::empty(),
        }
    }
}

///
/// Used to convert controls to views.
/// Controls can be consumed during conversion.
///
/// It will be called from ui!() macro.
///
/// Used to convert view models to views.
///
pub trait ViewModel {
    fn create_view(self: &Rc<Self>) -> Rc<dyn ControlObject>;

	fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject>
    where
        Self: Sized,
    {
        let view = Rc::new(self).create_view();
        view.get_context().set_attached_values(context.attached_values);
        view
    }
}
