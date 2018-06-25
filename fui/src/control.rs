use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use events::*;

pub enum HitTestResult {
    Nothing,
    Current,
    Child(Rc<RefCell<ControlObject>>)
}

pub trait Style<D> {
    fn get_preferred_size(&self, data: &D, drawing_context: &mut DrawingContext, size: Size) -> Size;
    fn set_rect(&mut self, data: &mut D, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, data: &D, point: Point) -> HitTestResult;

    fn to_primitives(&self, data: &D,
        drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}

pub trait Control {
    type Data;

    fn get_data(&self) -> &Self::Data;
    fn get_style(&self) -> &Box<Style<Self::Data>>;

    fn is_dirty(&self) -> bool;
    fn set_is_dirty(&mut self, is_dirty: bool);

    fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>>;
    fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>);
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>>;

    fn handle_event(&mut self, event: ControlEvent) -> bool;
}

pub trait ControlObject {
    fn is_dirty(&self) -> bool;
    fn set_is_dirty(&mut self, is_dirty: bool);

    fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>>;
    fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>);
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>>;
    fn handle_event(&mut self, event: ControlEvent) -> bool;

    // style related (cannot use Self /get_style() -> Style<Self::...>/ in trait object)
    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size;
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, point: Point) -> HitTestResult;

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
