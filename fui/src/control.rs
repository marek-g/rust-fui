use std::cell::RefCell;
use std::rc::Rc;

use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use events::*;

pub trait Style<P> {
    fn get_preferred_size(&self, properties: &P, drawing_context: &mut DrawingContext, size: Size) -> Size;
    fn set_rect(&mut self, properties: &mut P, rect: Rect);
    fn get_rect(&self) -> Rect;
    fn to_primitives(&self, properties: &P,
        drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}

pub trait Control {
    type Properties;

    fn get_properties(&self) -> &Self::Properties;
    fn get_style(&self) -> &Box<Style<Self::Properties>>;

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>>;
    fn is_hit_test_visible(&self) -> bool;
    fn handle_event(&mut self, event: ControlEvent) -> bool;
}

pub trait ControlObject {
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>>;
    fn is_hit_test_visible(&self) -> bool;
    fn handle_event(&mut self, event: ControlEvent) -> bool;

    // style related (cannot use Self /get_style() -> Style<Self::...>/ in trait object)
    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size;
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;
    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}

// This doesn't work, not sure why.
/*impl<P> ControlObject for Control<Properties = P> {

    fn set_rect(&mut self, rect: Rect) {
        self.set_rect(rect)
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        self.handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.get_style().get_preferred_size(self.get_properties(), drawing_context, size)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_style().to_primitives(&self.get_properties(),
            drawing_context, self.get_rect())
    }

}*/
