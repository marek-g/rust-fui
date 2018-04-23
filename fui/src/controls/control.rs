use common::size::Size;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;

pub trait Style<P> {
    fn get_preferred_size(&self, properties: &P, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn to_primitives<'a>(&self, properties: &'a P, size: Size, drawing_context: &mut DrawingContext) -> Vec<Primitive<'a>>;
}

pub trait Control {
    // style related
    // (I would like to see here get_style->Style<Self::Properties>,
    // but we cannot have "Self" in trait objects).
    fn get_preferred_size(&self, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn to_primitives(&self, size: Size, drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}
