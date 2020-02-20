use crate::style::default_theme::gradient_rect;
use drawing::primitive_extensions::pixel_rect_path;
use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use euclid::Length;
use fui::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct TextBox {
    pub text: Property<String>,
}

impl View for TextBox {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        Control::new(self, TextBoxDefaultStyle::new(), context)
    }
}

//
// TextBox Default Style
//

pub struct TextBoxDefaultStyle {
    rect: Rect,
    event_subscriptions: Vec<EventSubscription>,
    font_name: &'static str,
    font_size: u8,
}

impl TextBoxDefaultStyle {
    pub fn new() -> TextBoxDefaultStyle {
        TextBoxDefaultStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            event_subscriptions: Vec::new(),
            font_name: "OpenSans-Regular.ttf",
            font_size: 20u8,
        }
    }
}

impl Style<TextBox> for TextBoxDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut TextBox,
        control: &Rc<RefCell<Control<TextBox>>>,
    ) {
        self.event_subscriptions
            .push(data.text.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        _data: &mut TextBox,
        _context: &mut ControlContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut TextBox,
        _context: &mut ControlContext,
        resources: &mut dyn Resources,
        _size: Size,
    ) {
        let (text_width, text_height) = resources
            .get_font_dimensions(self.font_name, self.font_size, &data.text.get())
            .unwrap_or((0, 0));
        self.rect = Rect::new(
            0.0f32,
            0.0f32,
            text_width as f32 + 8.0f32,
            text_height as f32 + 8.0f32,
        )
    }

    fn set_rect(&mut self, _data: &mut TextBox, _context: &mut ControlContext, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &TextBox,
        _context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &TextBox,
        _context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let (text_width, text_height) = resources
            .get_font_dimensions(self.font_name, self.font_size, &data.text.get())
            .unwrap_or((0, 0));

        default_theme::border_3d_single(&mut vec, x, y, width, height, false, false);
        default_theme::border_3d_single(
            &mut vec,
            x + 2.0f32,
            y + 2.0f32,
            width - 4.0f32,
            height - 4.0f32,
            true,
            false,
        );
        vec.push(Primitive::Stroke {
            path: pixel_rect_path(
                PixelRect::new(
                    PixelPoint::new(x + 1.0f32, y + 1.0f32),
                    PixelSize::new(width - 2.0f32, height - 2.0f32),
                ),
                PixelThickness::new(1.0f32),
            ),
            thickness: PixelThickness::new(1.0f32),
            brush: drawing::primitive::Brush::Color {
                color: [0.4, 0.4, 0.4, 1.0],
            },
        });
        gradient_rect(
            &mut vec,
            x + 3.0f32,
            y + 3.0f32,
            width - 6.0f32,
            height - 6.0f32,
            [1.0, 1.0, 1.0, 0.6],
            [0.9, 0.9, 0.9, 0.6],
        );

        vec.push(Primitive::Text {
            resource_key: self.font_name.to_string(),
            color: [0.0, 0.0, 0.0, 1.0],
            position: PixelPoint::new(x + 4.0f32, y + (height - text_height as f32) / 2.0),
            clipping_rect: PixelRect::new(PixelPoint::new(x, y), PixelSize::new(width, height)),
            size: Length::new(self.font_size as f32),
            text: data.text.get(),
        });

        vec
    }
}
