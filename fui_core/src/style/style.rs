use crate::control::ControlContext;
use crate::FuiDrawingContext;

use crate::common::*;
use crate::control::*;
use crate::events::ControlEvent;
use crate::EventContext;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Style<D> {
    fn setup(&mut self, data: &mut D, control_context: &mut ControlContext);

    fn handle_event(
        &mut self,
        data: &mut D,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        event_context: &mut dyn EventContext,
        event: ControlEvent,
    );

    fn measure(
        &mut self,
        data: &mut D,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size;

    fn set_rect(
        &mut self,
        data: &mut D,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    );

    fn hit_test(
        &self,
        data: &D,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>>;

    fn draw(
        &mut self,
        data: &D,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    );
}
