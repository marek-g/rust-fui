use std::cell::RefCell;
use std::rc::Rc;

use typed_builder::TypedBuilder;

use super::stack_panel::*;
use fui::*;

#[derive(TypedBuilder)]
pub struct Vertical {}

impl Control for Vertical {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        StackPanel::builder()
            .orientation(Orientation::Vertical)
            .build()
            .to_view(context)
    }
}
