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
        StyledControl::new(self, TextBoxDefaultStyle::new(), context)
    }
}

//
// TextBox Default Style
//

pub struct TextBoxDefaultStyle {
    rect: Rect,
    is_focused: bool,
    event_subscriptions: Vec<EventSubscription>,
    font_name: &'static str,
    font_size: u8,

    cursor_pos_char: usize,
    cursor_pos_px: f32,
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
            is_focused: false,
            event_subscriptions: Vec::new(),
            font_name: "OpenSans-Regular.ttf",
            font_size: 20u8,

            cursor_pos_char: 0,
            cursor_pos_px: 0.0f32,
        }
    }

    fn calc_cursor_pos(
        &self,
        text: &str,
        pos: &Point,
        resources: &mut dyn Resources,
    ) -> (usize, f32) {
        let (char_widths, text_height) = resources
            .get_font_dimensions_each_char(self.font_name, self.font_size, &text)
            .unwrap_or((Vec::new(), 0));

        let pos = (pos.x - self.rect.x - 4.0f32) as i32;

        let mut cursor_char = 0;
        let mut cursor_px = 0;
        while cursor_char < char_widths.len()
            && pos >= cursor_px + char_widths[cursor_char] as i32 / 2
        {
            cursor_px += char_widths[cursor_char] as i32;
            cursor_char += 1;
        }

        (cursor_char, cursor_px as f32)
    }
}

impl Style<TextBox> for TextBoxDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut TextBox,
        control: &Rc<RefCell<StyledControl<TextBox>>>,
    ) {
        self.event_subscriptions
            .push(data.text.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        data: &mut TextBox,
        context: &mut ControlContext,
        resources: &mut dyn Resources,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::FocusEnter => {
                self.is_focused = true;
                context.set_is_dirty(true);
            }

            ControlEvent::FocusLeave => {
                self.is_focused = false;
                context.set_is_dirty(true);
            }

            ControlEvent::TapDown { ref position } => {
                let cursor_pos = self.calc_cursor_pos(&data.text.get(), position, resources);
                self.cursor_pos_char = cursor_pos.0;
                self.cursor_pos_px = cursor_pos.1;
            }

            _ => (),
        }
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

    fn hit_test(&self, _data: &TextBox, _context: &ControlContext, point: Point) -> HitTestResult {
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
                color: if self.is_focused {
                    [1.0, 1.0, 0.0, 1.0]
                } else {
                    [0.4, 0.4, 0.4, 1.0]
                },
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

        // draw cursor
        if self.is_focused {
            vec.push(Primitive::Rectangle {
                color: [1.0, 1.0, 0.0, 1.0],
                rect: PixelRect::new(
                    PixelPoint::new(
                        x + 4.0f32 + self.cursor_pos_px,
                        y + (height - text_height as f32) / 2.0,
                    ),
                    PixelSize::new(2.0f32, text_height as f32),
                ),
            });
        }

        vec
    }
}
