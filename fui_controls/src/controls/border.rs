use std::cell::RefCell;
use std::rc::Rc;

use fui_core::*;
use typed_builder::TypedBuilder;

use crate::style::*;
use fui_drawing::prelude::*;

pub enum BorderType {
    None,
    Sunken,
    Raisen,
    Frame3D,
}

#[derive(TypedBuilder)]
pub struct Border {
    #[builder(default = BorderType::Sunken)]
    border_type: BorderType,
}

impl Border {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultBorderStyle::new(
                    DefaultBorderStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default Border Style
//

const BORDER_SIZE: f32 = 1.0f32;

#[derive(TypedBuilder)]
pub struct DefaultBorderStyleParams {
    #[builder(default = Property::new([0.0f32, 0.0f32, 0.0f32, 0.0f32]))]
    background_color: Property<Color>,
}

pub struct DefaultBorderStyle {
    params: DefaultBorderStyleParams,
}

impl DefaultBorderStyle {
    pub fn new(params: DefaultBorderStyleParams) -> Self {
        DefaultBorderStyle { params }
    }

    fn get_border_size(data: &mut Border) -> f32 {
        match data.border_type {
            BorderType::None => 0f32,
            BorderType::Sunken | BorderType::Raisen => BORDER_SIZE,
            BorderType::Frame3D => BORDER_SIZE * 3.0f32,
        }
    }
}

impl Style<Border> for DefaultBorderStyle {
    fn setup(&mut self, _data: &mut Border, control_context: &mut ControlContext) {
        control_context.dirty_watch_property(&self.params.background_color);
    }

    fn handle_event(
        &mut self,
        _data: &mut Border,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Border,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        let children = control_context.get_children();

        let border_size = Self::get_border_size(data);

        let content_size = match children.into_iter().next() {
            Some(ref content) => {
                let child_size = Size::new(
                    if size.width.is_finite() {
                        0f32.max(size.width - border_size * 2.0f32)
                    } else {
                        size.width
                    },
                    if size.height.is_finite() {
                        0f32.max(size.height - border_size * 2.0f32)
                    } else {
                        size.height
                    },
                );
                content.borrow_mut().measure(drawing_context, child_size);
                let rect = content.borrow().get_rect();
                Size::new(rect.width, rect.height)
            }
            _ => Size::new(0f32, 0f32),
        };

        Size::new(
            content_size.width + border_size * 2.0f32,
            content_size.height + border_size * 2.0f32,
        )
    }

    fn set_rect(
        &mut self,
        data: &mut Border,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    ) {
        let border_size = Self::get_border_size(data);

        let content_rect = Rect::new(
            rect.x + border_size,
            rect.y + border_size,
            rect.width - border_size * 2.0f32,
            rect.height - border_size * 2.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(drawing_context, content_rect);
        }
    }

    fn hit_test(
        &self,
        _data: &Border,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if point.is_inside(&control_context.get_rect()) {
            let children = control_context.get_children();
            if let Some(ref content) = children.into_iter().next() {
                let c = content.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    let child_hit_test = c.hit_test(point);
                    if child_hit_test.is_some() {
                        return child_hit_test;
                    }
                }
            }
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        data: &Border,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let r = control_context.get_rect();

        let x = r.x;
        let y = r.y;
        let width = r.width;
        let height = r.height;

        let background_color = self.params.background_color.get();
        if background_color.alpha > 0.0f32 {
            drawing_context
                .display
                .draw_rect(rect(x, y, width, height), background_color);
        }

        match data.border_type {
            BorderType::None => (),

            BorderType::Sunken => default_theme::border_3d_single(
                &mut drawing_context.display,
                x,
                y,
                width,
                height,
                true,
                false,
                false,
            ),

            BorderType::Raisen => default_theme::border_3d_single(
                &mut drawing_context.display,
                x,
                y,
                width,
                height,
                false,
                false,
                false,
            ),

            BorderType::Frame3D => default_theme::border_3d_with_color(
                &mut drawing_context.display,
                x,
                y,
                width,
                height,
                true,
                true,
                default_theme::WINDOW_FRAME_COLOR.into(),
            ),
        }

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().draw(drawing_context);
        }
    }
}
