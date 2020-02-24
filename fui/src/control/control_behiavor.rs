use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;

use crate::common::*;
use crate::control::ControlObject;
use crate::events::*;
use crate::resources::Resources;

pub enum HitTestResult {
    Nothing,
    Current,
    Child(Rc<RefCell<dyn ControlObject>>),
}

pub trait ControlBehavior {
    fn handle_event(&mut self, event: ControlEvent);
    fn measure(&mut self, resources: &mut dyn Resources, size: Size);
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, point: Point) -> HitTestResult;

    fn to_primitives(&self, resources: &mut dyn Resources) -> Vec<Primitive>;
}
