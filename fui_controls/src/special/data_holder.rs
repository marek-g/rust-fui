use std::cell::RefCell;
use std::rc::Rc;

use crate::{Alignment, Margin};
use drawing::primitive::Primitive;
use fui_core::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct DataHolder<T> {
    pub data: T,
}

impl<T: 'static> DataHolder<T> {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultDataHolderStyle::new(
                    DefaultDataHolderStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default DataHolder Style
//

#[derive(TypedBuilder)]
pub struct DefaultDataHolderStyleParams {}

pub struct DefaultDataHolderStyle {
    rect: Rect,
}

impl DefaultDataHolderStyle {
    pub fn new(_params: DefaultDataHolderStyleParams) -> Self {
        DefaultDataHolderStyle {
            rect: Rect::empty(),
        }
    }
}

impl<T: 'static> Style<DataHolder<T>> for DefaultDataHolderStyle {
    fn setup(&mut self, _data: &mut DataHolder<T>, _control_context: &mut ControlContext) {}

    fn handle_event(
        &mut self,
        _data: &mut DataHolder<T>,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        _data: &mut DataHolder<T>,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        mut size: Size,
    ) {
        let map = control_context.get_attached_values();
        size = Margin::remove_from_size(size, &map);

        let children = control_context.get_children();
        let content_rect = if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(drawing_context, size);
            content.borrow().get_rect()
        } else {
            Rect::empty()
        };
        self.rect = content_rect;
        self.rect = Margin::add_to_rect(self.rect, &map);
    }

    fn set_rect(
        &mut self,
        _data: &mut DataHolder<T>,
        control_context: &mut ControlContext,
        rect: Rect,
    ) {
        let map = control_context.get_attached_values();
        Alignment::apply(
            &mut self.rect,
            rect,
            &map,
            Alignment::Stretch,
            Alignment::Stretch,
        );
        self.rect = Margin::remove_from_rect(self.rect, &map);

        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow_mut().set_rect(self.rect);
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
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
            };
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
