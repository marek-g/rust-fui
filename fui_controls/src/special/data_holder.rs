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

impl<T: 'static> Control for DataHolder<T> {
    fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
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

pub struct DefaultDataHolderStyle {
    rect: Rect,
}

impl DefaultDataHolderStyle {
    pub fn new(_params: DefaultDataHolderStyleParams) -> Self {
        DefaultDataHolderStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
        }
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
        _context: &mut ControlContext,
        _resources: &mut dyn Resources,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut DataHolder<T>,
        context: &mut ControlContext,
        resources: &mut dyn Resources,
        size: Size,
    ) {
        let children = context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow_mut().measure(resources, size);
            self.rect = child.borrow().get_rect();
        } else {
            self.rect = Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);
        }
    }

    fn set_rect(&mut self, data: &mut DataHolder<T>, context: &mut ControlContext, rect: Rect) {
        self.rect = rect;
        let children = context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow_mut().set_rect(rect);
        }
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &DataHolder<T>,
        context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        let children = context.get_children();
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
        context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let children = context.get_children();
        if let Some(child) = children.into_iter().next() {
            vec.append(&mut child.borrow().to_primitives(resources));
        }

        vec
    }
}
