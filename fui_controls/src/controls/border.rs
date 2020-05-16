use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use fui::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct Border {}

impl Control for Border {
    fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
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
        _context: &mut ControlContext,
        _resources: &mut dyn Resources,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        _data: &mut Border,
        context: &mut ControlContext,
        resources: &mut dyn Resources,
        size: Size,
    ) {
        let children = context.get_children();

        let content_size = if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(resources, size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };
        self.rect = Rect::new(
            0.0f32,
            0.0f32,
            content_size.width + 2.0f32,
            content_size.height + 2.0f32,
        )
    }

    fn set_rect(&mut self, _data: &mut Border, context: &mut ControlContext, rect: Rect) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + 1.0f32,
            rect.y + 1.0f32,
            rect.width - 2.0f32,
            rect.height - 2.0f32,
        );

        let children = context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Border, context: &ControlContext, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            let children = context.get_children();
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
        context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        default_theme::border_3d_single(&mut vec, x, y, width, height, true, false, false);

        let children = context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let mut vec2 = content.borrow_mut().to_primitives(resources);
            vec.append(&mut vec2);
        }

        vec
    }
}
