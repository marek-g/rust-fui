use common::size::Size;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;

pub trait Style<S> {
    fn get_preferred_size(&self, state: &S, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn to_primitives(&self, state: &S, size: Size, drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}

pub trait Control {
    fn get_preferred_size(&self, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn to_primitives(&self, size: Size, drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}
