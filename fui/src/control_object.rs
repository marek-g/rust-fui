use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use common::*;
use control::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use events::*;

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

impl<D: 'static> ControlObject for Control<D> where Control<D>: ControlBehaviour {
    fn is_dirty(&self) -> bool {
        self.is_dirty()
    }

    fn set_is_dirty(&mut self, is_dirty: bool) {
        self.set_is_dirty(is_dirty)
    }

    fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>> {
        self.get_parent()
    }

    fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>) {
        self.set_parent(parent);
    }

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        (self as &mut ControlBehaviour).get_children()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        (self as &mut ControlBehaviour).handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.style.get_preferred_size(&self.data, drawing_context, size)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.style.set_rect(&self.data, rect);
    }

    fn get_rect(&self) -> Rect {
        self.style.get_rect()
    }

    fn hit_test(&self, point: Point) -> HitTestResult {
        self.style.hit_test(&self.data, point)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.style.to_primitives(&self.data, drawing_context)
    }
}
