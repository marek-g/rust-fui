use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct DataHolder<T> {
    pub data: T,
}

impl<T: 'static> DataHolder<T> {
    pub fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(self,
            style.unwrap_or_else(|| {
                Box::new(DefaultDataHolderStyle::new(DefaultDataHolderStyleParams::builder().build()))
            }),
            context)
    }
}

//
// Default DataHolder Style
//


#[derive(TypedBuilder)]
pub struct DefaultDataHolderStyleParams {}

pub struct DefaultDataHolderStyle;

impl DefaultDataHolderStyle {
    pub fn new(_params: DefaultDataHolderStyleParams) -> Self {
        DefaultDataHolderStyle {}
    }
}

impl<T: 'static> Style<DataHolder<T>> for DefaultDataHolderStyle {
    fn setup_dirty_watching(
        &mut self,
        _data: &mut DataHolder<T>,
        _control: &Rc<RefCell<StyledControl<DataHolder<T>>>>,
    ) {
    }

    fn handle_event(
        &mut self,
        _data: &mut DataHolder<T>,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut DataHolder<T>,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow_mut().measure(drawing_context, size);
        }
    }

    fn set_rect(&mut self, data: &mut DataHolder<T>, control_context: &mut ControlContext, rect: Rect) {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow_mut().set_rect(rect);
        }
    }

    fn get_rect(&self, control_context: &ControlContext) -> Rect {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow().get_rect()
        } else {
            Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32)
        }
    }

    fn hit_test(
        &self,
        _data: &DataHolder<T>,
        control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            let child_hit_test = child.borrow_mut().hit_test(point);
            return match child_hit_test {
                HitTestResult::Current => HitTestResult::Child(child.clone()),
                HitTestResult::Child(..) => child_hit_test,
                HitTestResult::Nothing => HitTestResult::Nothing,
            }
        }
        HitTestResult::Nothing
    }

    fn to_primitives(
        &self,
        _data: &DataHolder<T>,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow().to_primitives(drawing_context)
        } else {
            (Vec::new(), Vec::new())
        }
    }
}
