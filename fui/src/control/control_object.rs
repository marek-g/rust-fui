use std::cell::RefCell;
use std::rc::{Rc, Weak};
use typemap::TypeMap;

use drawing::primitive::Primitive;

use crate::children_source::*;
use crate::common::*;
use crate::events::*;
use crate::resources::Resources;

pub enum HitTestResult {
    Nothing,
    Current,
    Child(Rc<RefCell<dyn ControlObject>>),
}

pub trait ControlObject {
    fn is_dirty(&self) -> bool;
    fn set_is_dirty(&mut self, is_dirty: bool);

    fn get_attached_values(&self) -> &TypeMap;

    fn get_parent(&self) -> Option<Rc<RefCell<dyn ControlObject>>>;
    fn set_parent(&mut self, parent: Weak<RefCell<dyn ControlObject>>);
    fn get_children(&mut self) -> &Box<dyn ChildrenSource>;

    // style related (cannot use Self /get_style() -> Style<Self::...>/ in trait object)
    fn handle_event(&mut self, event: ControlEvent);
    fn measure(&mut self, resources: &mut dyn Resources, size: Size);
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, point: Point) -> HitTestResult;

    fn to_primitives(&self, resources: &mut dyn Resources) -> Vec<Primitive>;
}
