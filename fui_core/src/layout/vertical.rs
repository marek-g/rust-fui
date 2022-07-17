use std::cell::RefCell;
use std::rc::Rc;

use typed_builder::TypedBuilder;

use super::stack_panel::*;
use crate::{ControlObject, Orientation, Style, ViewContext};

#[derive(TypedBuilder)]
pub struct Vertical {}

impl Vertical {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        StackPanel::builder()
            .orientation(Orientation::Vertical)
            .build()
            .to_view(None, context)
    }
}
