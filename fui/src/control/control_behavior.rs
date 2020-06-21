use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;

use crate::common::*;
use crate::control::ControlObject;
use crate::events::*;
use crate::DrawingContext;

pub enum HitTestResult {
    Nothing,
    Current,
    Child(Rc<RefCell<dyn ControlObject>>),
}

pub trait ControlBehavior {
    fn handle_event(&mut self, drawing_context: &mut dyn DrawingContext, event_context: &mut dyn EventContext, event: ControlEvent);
    fn measure(&mut self, drawing_context: &mut dyn DrawingContext, size: Size);
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, point: Point) -> HitTestResult;

    /// Returns primitives.
    /// First vector contains primitives for normal layer (most controls).
    /// Second vector contains primitives for overlay layer (used by popup / menu etc.).
    fn to_primitives(&self, drawing_context: &mut dyn DrawingContext) -> (Vec<Primitive>, Vec<Primitive>);
}
