use common::size::Size;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;

pub trait Style<C: Control> {
    fn get_preferred_size(&self, control: &C, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn to_primitives(&self, control: &C, size: Size, drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}

pub trait Control {
    fn get_style(&self) -> &Style<Self>;
}
