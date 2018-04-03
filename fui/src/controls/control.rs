use common::size::Size;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;

pub trait Control {
    fn get_preferred_size(&mut self, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn set_size(&mut self, size: Size,  drawing_context: &mut DrawingContext) -> Size;

    fn to_primitives(&self) -> Vec<Primitive>;
}