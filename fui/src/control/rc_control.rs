use crate::children_source::ChildrenSource;
use crate::common::*;
use crate::control::control_object::*;
use crate::control::styled_control::HitTestResult;
use crate::events::ControlEvent;
use crate::resources::Resources;
use drawing::primitive::Primitive;
use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

pub trait RcControl {
    fn is_dirty(self_rc: &Rc<RefCell<Self>>) -> bool;
    fn set_is_dirty(self_rc: &Rc<RefCell<Self>>, is_dirty: bool);

    fn get_attached_values(self_rc: &Rc<RefCell<Self>>) -> &TypeMap;

    fn get_parent(self_rc: &Rc<RefCell<Self>>) -> Option<Box<dyn ControlObject>>;
    fn set_parent(self_rc: &Rc<RefCell<Self>>, parent: Box<dyn WeakControlObject>);
    fn get_children(self_rc: &Rc<RefCell<Self>>) -> &Box<dyn ChildrenSource>;

    // style related (cannot use Self /get_style() -> Style<Self::...>/ in trait object)
    fn handle_event(self_rc: &Rc<RefCell<Self>>, event: ControlEvent);
    fn measure(self_rc: &Rc<RefCell<Self>>, resources: &mut dyn Resources, size: Size);
    fn set_rect(self_rc: &Rc<RefCell<Self>>, rect: Rect);
    fn get_rect(self_rc: &Rc<RefCell<Self>>) -> Rect;

    fn hit_test(self_rc: &Rc<RefCell<Self>>, point: Point) -> HitTestResult;

    fn to_primitives(self_rc: &Rc<RefCell<Self>>, resources: &mut dyn Resources) -> Vec<Primitive>;
}
