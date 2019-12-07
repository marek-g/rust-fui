use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use drawing::primitive::Primitive;

use crate::children_source::*;
use crate::common::*;
use crate::control::*;
use crate::events::ControlEvent;
use crate::observable::*;
use crate::resources::Resources;

pub trait Style<D> {
    fn setup_dirty_watching(&mut self, data: &mut D, control: &Rc<RefCell<Control<D>>>);

    fn handle_event(
        &mut self,
        data: &mut D,
        children: &Box<dyn ChildrenSource>,
        event: ControlEvent,
    );

    fn measure(
        &mut self,
        data: &mut D,
        children: &Box<dyn ChildrenSource>,
        resources: &mut dyn Resources,
        size: Size,
    );
    fn set_rect(&mut self, data: &mut D, children: &Box<dyn ChildrenSource>, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, data: &D, children: &Box<dyn ChildrenSource>, point: Point)
        -> HitTestResult;

    fn to_primitives(
        &self,
        data: &D,
        children: &Box<dyn ChildrenSource>,
        resources: &mut dyn Resources,
    ) -> Vec<Primitive>;
}

pub trait PropertyDirtyExtension<D> {
    fn dirty_watching(&mut self, control: &Rc<RefCell<Control<D>>>) -> EventSubscription;
}

impl<D: 'static, T> PropertyDirtyExtension<D> for Property<T>
where
    T: 'static + Clone + PartialEq + Default,
{
    fn dirty_watching(&mut self, control: &Rc<RefCell<Control<D>>>) -> EventSubscription {
        let weak_control = Rc::downgrade(control);
        self.on_changed(move |_| {
            weak_control
                .upgrade()
                .map(|control| (control.borrow_mut() as RefMut<Control<D>>).set_is_dirty(true));
        })
    }
}
