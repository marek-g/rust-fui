use typed_builder::TypedBuilder;

use super::stack_panel::*;
use crate::{Children, Orientation, Style, ViewContext};

#[derive(TypedBuilder)]
pub struct Vertical {}

impl Vertical {
    pub fn to_view(self, _style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Children {
        StackPanel::builder()
            .orientation(Orientation::Vertical)
            .build()
            .to_view(None, context)
    }
}
