use std::cell::RefCell;
use std::rc::Rc;

use typed_builder::TypedBuilder;

use super::stack_panel::*;
use common::Orientation;
use control_object::*;
use view::*;

#[derive(TypedBuilder)]
pub struct Vertical {}

impl View for Vertical {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        StackPanel::builder()
            .orientation(Orientation::Vertical)
            .build()
            .to_view(context)
    }
}
