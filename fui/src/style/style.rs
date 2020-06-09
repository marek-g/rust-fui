use crate::control::ControlContext;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use drawing::primitive::Primitive;

use crate::common::*;
use crate::control::*;
use crate::events::ControlEvent;
use crate::observable::*;
use crate::{DrawingContext, drawing::Resources};

pub trait Style<D> {
    fn setup_dirty_watching(&mut self, data: &mut D, control: &Rc<RefCell<StyledControl<D>>>);

    fn handle_event(
        &mut self,
        data: &mut D,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        event: ControlEvent,
    );

    fn measure(
        &mut self,
        data: &mut D,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    );
    fn set_rect(&mut self, data: &mut D, control_context: &mut ControlContext, rect: Rect);
    fn get_rect(&self, control_context: &ControlContext) -> Rect;

    fn hit_test(&self, data: &D, control_context: &ControlContext, point: Point) -> HitTestResult;

    fn to_primitives(
        &self,
        data: &D,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
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
