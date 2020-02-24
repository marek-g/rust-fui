use crate::control::control_context::ControlContext;
use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;

use crate::common::*;
use crate::events::*;
use crate::resources::Resources;

pub enum HitTestResult {
    Nothing,
    Current,
    Child(Rc<RefCell<dyn ControlObject>>),
}

pub trait ControlObject {
    fn get_context(&self) -> &ControlContext;
    fn get_context_mut(&mut self) -> &mut ControlContext;

    // style related (cannot use Self /get_style() -> Style<Self::...>/ in trait object)
    fn handle_event(&mut self, event: ControlEvent);
    fn measure(&mut self, resources: &mut dyn Resources, size: Size);
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, point: Point) -> HitTestResult;

    fn to_primitives(&self, resources: &mut dyn Resources) -> Vec<Primitive>;
}
