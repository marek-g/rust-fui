use crate::control::ControlContext;

use drawing::primitive::Primitive;

use crate::common::*;
use crate::control::*;
use crate::events::ControlEvent;
use crate::{DrawingContext, EventContext};

pub trait Style<D> {
    fn setup(&mut self, data: &mut D, control_context: &mut ControlContext);

    fn handle_event(
        &mut self,
        data: &mut D,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        event_context: &mut dyn EventContext,
        event: ControlEvent,
    );

    fn measure(
        &mut self,
        data: &mut D,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    );
    fn set_rect(&mut self, data: &mut D, control_context: &mut ControlContext, rect: Rect);
    fn get_rect(&self, control_context: &ControlContext) -> Rect;

    fn hit_test(&self, data: &D, control_context: &ControlContext, point: Point) -> HitTestResult;

    fn to_primitives(
        &self,
        data: &D,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>);
}
