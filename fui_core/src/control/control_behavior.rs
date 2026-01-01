use std::cell::RefCell;
use std::rc::Rc;

use crate::common::*;
use crate::control::ControlObject;
use crate::events::*;
use crate::FuiDrawingContext;

pub trait ControlBehavior {
    fn setup(&mut self);

    fn handle_event(
        &mut self,
        drawing_context: &mut FuiDrawingContext,
        event_context: &mut dyn EventContext,
        event: ControlEvent,
    );
    fn measure(&mut self, drawing_context: &mut FuiDrawingContext, size: Size);
    fn set_rect(&mut self, drawing_context: &mut FuiDrawingContext, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, point: Point) -> Option<Rc<RefCell<dyn ControlObject>>>;

    /// Draws control content.
    fn draw(&mut self, drawing_context: &mut FuiDrawingContext);
}
