use std::cell::RefCell;
use std::rc::Rc;

use typed_builder::TypedBuilder;

use super::stack_panel::*;
use fui::*;

#[derive(TypedBuilder)]
pub struct Horizontal {}

impl Horizontal {
    pub fn to_view(self, _style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        StackPanel::builder()
            .orientation(Orientation::Horizontal)
            .build()
            .to_view(None, context)
    }
}
