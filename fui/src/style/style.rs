use crate::control::ControlContext;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use drawing::primitive::Primitive;

use crate::common::*;
use crate::control::*;
use crate::events::ControlEvent;
use crate::observable::*;
use crate::resources::Resources;

pub trait Style<D> {
    fn setup_dirty_watching(&mut self, data: &mut D, control: &Rc<RefCell<StyledControl<D>>>);

    fn handle_event(
        &mut self,
        data: &mut D,
        context: &mut ControlContext,
        resources: &mut dyn Resources,
        event: ControlEvent,
    );

    fn measure(
        &mut self,
        data: &mut D,
        context: &mut ControlContext,
        resources: &mut dyn Resources,
        size: Size,
    );
    fn set_rect(&mut self, data: &mut D, context: &mut ControlContext, rect: Rect);
    fn get_rect(&self, context: &ControlContext) -> Rect;

    fn hit_test(&self, data: &D, context: &ControlContext, point: Point) -> HitTestResult;

    fn to_primitives(
        &self,
        data: &D,
        context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> (Vec<Primitive>, Vec<Primitive>);
}

pub trait PropertyDirtyExtension<D> {
    fn dirty_watching(&self, control: &Rc<RefCell<StyledControl<D>>>) -> EventSubscription;
}

impl<D: 'static, T> PropertyDirtyExtension<D> for Property<T>
where
    T: 'static + Clone + PartialEq,
{
    fn dirty_watching(&self, control: &Rc<RefCell<StyledControl<D>>>) -> EventSubscription {
        let weak_control = Rc::downgrade(control);
        self.on_changed(move |_| {
            weak_control.upgrade().map(|control| {
                (control.borrow_mut() as RefMut<StyledControl<D>>)
                    .get_context_mut()
                    .set_is_dirty(true)
            });
        })
    }
}
