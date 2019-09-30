use std::cell::RefCell;
use std::rc::Rc;

use typed_builder::TypedBuilder;

use common::Orientation;
use control_object::*;
use view::*;
use super::stack_panel::*;

#[derive(TypedBuilder)]
pub struct Vertical {}

impl View for Vertical {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>> {
        StackPanel::builder().orientation(Orientation::Vertical).build().to_view(context)
    }
}
