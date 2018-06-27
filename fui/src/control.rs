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

    fn get_control_common(&self) -> &ControlCommon;
    fn get_control_common_mut(&mut self) -> &mut ControlCommon;

    fn get_data(&self) -> &Self::Data;
    fn get_style(&self) -> &Box<Style<Self::Data>>;
    fn get_style_and_data_mut(&mut self) -> (&mut Box<Style<Self::Data>>, &Self::Data);

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>>;

    fn handle_event(&mut self, event: ControlEvent) -> bool;
}

pub struct ControlCommon {
    parent: Option<Weak<RefCell<ControlObject>>>,
    is_dirty: bool,
}

impl ControlCommon {
    pub fn new() -> Self {
        ControlCommon {
            parent: None,
            is_dirty: true,
        }
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>> {
        if let Some(ref test) = self.parent {
            test.upgrade()
        } else {
            None
        }
    }

    pub fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>) {
        self.parent = Some(parent);
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    
    pub fn set_is_dirty(&mut self, is_dirty: bool) {
        self.is_dirty = is_dirty;
        if is_dirty {
            if let Some(ref parent) = self.get_parent() {
                parent.borrow_mut().set_is_dirty(is_dirty)
            }
        }
    }
}
