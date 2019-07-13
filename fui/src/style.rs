use std::cell::{ RefCell, RefMut };
use std::rc::Rc;

use common::*;
use control::*;
use control_object::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use events::ControlEvent;
use observable::*;

pub trait Style<D> {
    fn setup_dirty_watching(&mut self, data: &mut D, control: &Rc<RefCell<Control<D>>>);

    fn handle_event(&mut self, data: &mut D, children: &Vec<Rc<RefCell<ControlObject>>>, event: ControlEvent);

    fn get_preferred_size(&self, data: &D, children: &Vec<Rc<RefCell<ControlObject>>>,
        drawing_context: &mut DrawingContext, size: Size) -> Size;
    fn set_rect(&mut self, data: &D, children: &Vec<Rc<RefCell<ControlObject>>>,
        rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, data: &D, children: &Vec<Rc<RefCell<ControlObject>>>,
        point: Point) -> HitTestResult;

    fn to_primitives(&self, data: &D, children: &Vec<Rc<RefCell<ControlObject>>>,
        drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}

pub trait PropertyDirtyExtension<D> {
    fn dirty_watching(&mut self, control: &Rc<RefCell<Control<D>>>) -> EventSubscription;
}

impl<D: 'static, T> PropertyDirtyExtension<D> for Property<T>
    where T: 'static + Clone + PartialEq + Default {
    fn dirty_watching(&mut self, control: &Rc<RefCell<Control<D>>>) -> EventSubscription {
        let weak_control = Rc::downgrade(control);
        self.on_changed(move |_| {
            weak_control.upgrade().map(|control| (control.borrow_mut() as RefMut<Control<D>>).set_is_dirty(true));
        })
    }
}
