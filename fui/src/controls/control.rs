use common::size::Size;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16
}

pub trait Style<P> {
    fn get_preferred_size(&self, properties: &P, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn to_primitives<'a>(&self, properties: &'a P,
        rect: Rect,
        drawing_context: &mut DrawingContext) -> Vec<Primitive<'a>>;
}

pub trait Control {
    type Properties;

    fn get_properties(&self) -> &Self::Properties;
    fn get_syle(&self) -> &Box<Style<Self::Properties>>;

    fn set_size(&mut self, rect: Rect);
    fn get_size(&self) -> Rect;

    fn handle_event(&mut self, event: &::winit::Event) -> bool;
}

pub trait ControlObject {
    fn set_size(&mut self, rect: Rect);
    fn handle_event(&mut self, event: &::winit::Event) -> bool;

    // style related
    // (I would like to see here get_style->Style<Self::Properties>,
    // but we cannot have "Self" in trait objects).
    fn get_preferred_size(&self, size: Size, drawing_context: &mut DrawingContext) -> Size;
    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}

// This doesn't work, not sure why.
/*impl<P> ControlObject for Control<Properties = P> {

    fn set_size(&mut self, rect: Rect) {
        self.set_size(rect)
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        self.handle_event(event)
    }

    fn get_preferred_size(&self, size: Size, drawing_context: &mut DrawingContext) -> Size {
        self.get_syle().get_preferred_size(self.get_properties(), size, drawing_context)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_syle().to_primitives(&self.get_properties(),
            self.get_size(),
            drawing_context)
    }

}*/
