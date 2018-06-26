use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use common::*;
use control_object::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use events::*;

pub enum HitTestResult {
    Nothing,
    Current,
    Child(Rc<RefCell<ControlObject>>)
}

pub trait Style<D> {
    fn get_preferred_size(&self, data: &D, drawing_context: &mut DrawingContext, size: Size) -> Size;
    fn set_rect(&mut self, data: &D, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, data: &D, point: Point) -> HitTestResult;

    fn to_primitives(&self, data: &D,
        drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}

pub trait Control {
    type Data;

    fn get_data(&self) -> &Self::Data;
    fn get_style(&self) -> &Box<Style<Self::Data>>;
    fn get_style_and_data_mut(&mut self) -> (&mut Box<Style<Self::Data>>, &Self::Data);

    fn is_dirty(&self) -> bool;
    fn set_is_dirty(&mut self, is_dirty: bool);

    fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>>;
    fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>);
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>>;

    fn handle_event(&mut self, event: ControlEvent) -> bool;
}
