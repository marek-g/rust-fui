use std::cell::RefCell;
use std::rc::{Rc, Weak};
use typemap::TypeMap;

use drawing::primitive::Primitive;

use crate::children_source::*;
use crate::common::*;
use crate::control::*;
use crate::events::*;
use crate::resources::Resources;

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

impl<D: 'static> ControlObject for StyledControl<D> {
    fn is_dirty(&self) -> bool {
        self.get_context().is_dirty()
    }

    fn set_is_dirty(&mut self, is_dirty: bool) {
        self.get_context_mut().set_is_dirty(is_dirty)
    }

    fn get_attached_values(&self) -> &TypeMap {
        self.get_context().get_attached_values()
    }

    fn get_parent(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        self.get_context().get_parent()
    }

    fn set_parent(&mut self, parent: Weak<RefCell<dyn ControlObject>>) {
        self.get_context_mut().set_parent(parent);
    }

    fn get_children(&mut self) -> &Box<dyn ChildrenSource> {
        self.get_context_mut().get_children()
    }

    fn handle_event(&mut self, event: ControlEvent) {
        self.style
            .handle_event(&mut self.data, &mut self.context, event)
    }

    fn measure(&mut self, resources: &mut dyn Resources, size: Size) {
        self.style
            .measure(&mut self.data, &mut self.context, resources, size)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.style.set_rect(&mut self.data, &mut self.context, rect);
    }

    fn get_rect(&self) -> Rect {
        self.style.get_rect()
    }

    fn hit_test(&self, point: Point) -> HitTestResult {
        self.style.hit_test(&self.data, &self.context, point)
    }

    fn to_primitives(&self, resources: &mut dyn Resources) -> Vec<Primitive> {
        self.style
            .to_primitives(&self.data, &self.context, resources)
    }
}
