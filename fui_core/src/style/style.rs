use crate::control::ControlContext;
use crate::FuiDrawingContext;

use crate::common::*;
use crate::control::*;
use crate::events::ControlEvent;
use crate::EventContext;
use std::rc::Rc;

pub trait Style<D> {
    // initialization - bottom-up
    fn setup(&mut self, data: &mut D, control_context: &ControlContext);

    // parent attached - top-down
    fn parent_attached(&mut self, _data: &mut D, _control_context: &ControlContext) {}

    // parent detached - top-down
    fn parent_detached(&mut self, _data: &mut D, _control_context: &ControlContext) {}

    fn handle_event(
        &mut self,
        data: &mut D,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        event_context: &mut dyn EventContext,
        event: ControlEvent,
    );

    fn measure(
        &mut self,
        data: &mut D,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size;

    fn set_rect(
        &mut self,
        data: &mut D,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    );

    fn hit_test(
        &self,
        data: &D,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<dyn ControlObject>>;

    fn draw(
        &mut self,
        data: &D,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    );
}
