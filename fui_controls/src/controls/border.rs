use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use fui::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct Border {}

impl Border {
    pub fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(self,
            style.unwrap_or_else(|| {
                Box::new(DefaultBorderStyle::new(DefaultBorderStyleParams::builder().build()))
            }),
            context)
    }
}

//
// Default Border Style
//

const BORDER_SIZE: f32 = 1.0f32;

#[derive(TypedBuilder)]
pub struct DefaultBorderStyleParams {}

pub struct DefaultBorderStyle {
    rect: Rect,
}

impl DefaultBorderStyle {
    pub fn new(_params: DefaultBorderStyleParams) -> Self {
        DefaultBorderStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
        }
    }
}

impl Style<Border> for DefaultBorderStyle {
    fn setup_dirty_watching(
        &mut self,
        _data: &mut Border,
        _control: &Rc<RefCell<StyledControl<Border>>>,
    ) {
    }

    fn handle_event(
        &mut self,
        _data: &mut Border,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        _data: &mut Border,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
        let children = control_context.get_children();

        let content_size = if let Some(ref content) = children.into_iter().next() {
            let child_size = Size::new(
                if size.width.is_finite() { 0f32.max(size.width - BORDER_SIZE * 2.0f32) } else { size.width },
                if size.height.is_finite() { 0f32.max(size.height - BORDER_SIZE * 2.0f32) } else { size.height },
            );
            content.borrow_mut().measure(drawing_context, child_size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };

        self.rect = Rect::new(
            0.0f32,
            0.0f32,
            content_size.width + BORDER_SIZE * 2.0f32,
            content_size.height + BORDER_SIZE * 2.0f32,
        )
    }

    fn set_rect(&mut self, _data: &mut Border, control_context: &mut ControlContext, rect: Rect) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + BORDER_SIZE,
            rect.y + BORDER_SIZE,
            rect.width - BORDER_SIZE * 2.0f32,
            rect.height - BORDER_SIZE * 2.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Border, control_context: &ControlContext, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            let children = control_context.get_children();
            if let Some(ref content) = children.into_iter().next() {
                let c = content.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    let child_hit_test = c.hit_test(point);
                    match child_hit_test {
                        HitTestResult::Current => return HitTestResult::Child(content.clone()),
                        HitTestResult::Child(..) => return child_hit_test,
                        HitTestResult::Nothing => (),
                    }
                }
            }
            HitTestResult::Nothing
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        _data: &Border,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        default_theme::border_3d_single(&mut vec, x, y, width, height, true, false, false);

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let (mut vec2, mut overlay2) = content.borrow_mut().to_primitives(drawing_context);
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}
