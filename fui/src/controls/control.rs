use common::size::Size;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;

pub trait Style<P> {
    fn get_preferred_size(&self, properties: &P, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn to_primitives<'a>(&self, properties: &'a P,
        x: u16, y: u16, width: u16, height: u16,
        drawing_context: &mut DrawingContext) -> Vec<Primitive<'a>>;
}

pub trait Control {
    fn update_size(&mut self, x: u16, y: u16, width: u16, height: u16);
    fn handle_event(&mut self, event: &::winit::Event) -> bool;

    // style related
    // (I would like to see here get_style->Style<Self::Properties>,
    // but we cannot have "Self" in trait objects).
    fn get_preferred_size(&self, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}
