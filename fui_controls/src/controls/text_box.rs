use std::cell::RefCell;
use std::rc::Rc;

use crate::style::default_theme::gradient_rect;
use drawing::clipping::Clipping;
use drawing::primitive::Primitive;
use drawing::transformation::Transformation;
use drawing::units::{PixelPoint, PixelRect, PixelSize};
use euclid::Length;
use fui_core::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct TextBox {
    pub text: Property<String>,
}

impl TextBox {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultTextBoxStyle::new(
                    DefaultTextBoxStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default TextBox Style
//

#[derive(TypedBuilder)]
pub struct DefaultTextBoxStyleParams {}

pub struct DefaultTextBoxStyle {
    is_hover: bool,
    is_focused: bool,
    event_subscriptions: Vec<EventSubscription>,
    font_name: &'static str,
    font_size: u8,

    cursor_pos_char: usize,
    cursor_pos_px: f32,
    offset_x: f32,
}

impl DefaultTextBoxStyle {
    pub fn new(_params: DefaultTextBoxStyleParams) -> Self {
        DefaultTextBoxStyle {
            is_hover: false,
            is_focused: false,
            event_subscriptions: Vec::new(),
            font_name: "OpenSans-Regular.ttf",
            font_size: 20u8,

            cursor_pos_char: 0,
            cursor_pos_px: 0.0f32,
            offset_x: 0.0f32,
        }
    }

    fn calc_cursor_pos(
        &self,
        text: &str,
        pos: &Point,
        rect: Rect,
        resources: &mut dyn Resources,
    ) -> (usize, f32) {
        let (char_widths, _) = resources
            .get_font_dimensions_each_char(self.font_name, self.font_size, &text)
            .unwrap_or((Vec::new(), 0));

        let pos = (pos.x - rect.x - 4.0f32) as i32;

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

    fn calc_cursor_pos_px(
        &self,
        text: &str,
        cursor_pos_char: usize,
        resources: &mut dyn Resources,
    ) -> f32 {
        let subtext: String = text.chars().take(cursor_pos_char).collect();
        let (text_width, _) = resources
            .get_font_dimensions(self.font_name, self.font_size, &subtext)
            .unwrap_or((0, 0));
        text_width as f32
    }

    fn move_cursor(
        &mut self,
        text: &str,
        cursor_pos_char: usize,
        rect: Rect,
        resources: &mut dyn Resources,
    ) {
        let cursor_pos_px = self.calc_cursor_pos_px(&text, cursor_pos_char, resources);
        self.cursor_pos_char = cursor_pos_char;
        self.cursor_pos_px = cursor_pos_px;

        self.update_offset_x(rect);
    }

    fn insert_str(
        &mut self,
        data: &mut TextBox,
        text: &str,
        rect: Rect,
        resources: &mut dyn Resources,
    ) {
        let t = data.text.get();
        let t: String = t
            .chars()
            .take(self.cursor_pos_char)
            .chain(text.chars())
            .chain(t.chars().skip(self.cursor_pos_char))
            .collect();

        let new_cursor_pos_char = self.cursor_pos_char + text.chars().count();

        self.move_cursor(&t, new_cursor_pos_char, rect, resources);
        data.text.set(t);
    }

    fn remove_char(
        &mut self,
        data: &mut TextBox,
        pos: usize,
        rect: Rect,
        resources: &mut dyn Resources,
    ) {
        let t = data.text.get();
        let t: String = t.chars().take(pos).chain(t.chars().skip(pos + 1)).collect();

        if pos < self.cursor_pos_char {
            self.move_cursor(&t, self.cursor_pos_char - 1, rect, resources);
        }

        data.text.set(t);
    }

    fn update_offset_x(&mut self, rect: Rect) {
        if self.is_focused {
            if self.cursor_pos_px < self.offset_x {
                self.offset_x = self.cursor_pos_px;
            } else if self.cursor_pos_px > self.offset_x + rect.width - 8.0f32 {
                self.offset_x = self.cursor_pos_px - rect.width + 8.0f32 + 2.0f32;
            }
        } else {
            self.offset_x = 0.0f32;
        }
    }
}

impl Style<TextBox> for DefaultTextBoxStyle {
    fn setup(&mut self, data: &mut TextBox, control_context: &mut ControlContext) {
        self.event_subscriptions
            .push(data.text.dirty_watching(&control_context.get_self_rc()));
    }

    fn handle_event(
        &mut self,
        data: &mut TextBox,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::FocusChange(value) => {
                self.is_focused = value;
                control_context.set_is_dirty(true);
            }

            ControlEvent::HoverChange(value) => {
                self.is_hover = value;
                control_context.set_is_dirty(true);
            }

            ControlEvent::TapDown { ref position } => {
                let cursor_pos = self.calc_cursor_pos(
                    &data.text.get(),
                    position,
                    control_context.get_rect(),
                    drawing_context.get_resources(),
                );
                self.cursor_pos_char = cursor_pos.0;
                self.cursor_pos_px = cursor_pos.1;
                control_context.set_is_dirty(true);
            }

            ControlEvent::KeyboardInput(ref key_event) => {
                if key_event.state == KeyState::Pressed {
                    if let Some(ref key_code) = key_event.keycode {
                        match key_code {
                            Keycode::Backspace => {
                                if self.cursor_pos_char > 0 {
                                    self.remove_char(
                                        data,
                                        self.cursor_pos_char - 1,
                                        control_context.get_rect(),
                                        drawing_context.get_resources(),
                                    );
                                }
                            }
                            Keycode::Delete => {
                                let text = data.text.get();
                                if self.cursor_pos_char < text.chars().count() {
                                    self.remove_char(
                                        data,
                                        self.cursor_pos_char,
                                        control_context.get_rect(),
                                        drawing_context.get_resources(),
                                    );
                                }
                            }
                            Keycode::Home => {
                                let text = data.text.get();
                                if self.cursor_pos_char > 0 {
                                    self.move_cursor(
                                        &text,
                                        0,
                                        control_context.get_rect(),
                                        drawing_context.get_resources(),
                                    );
                                }
                            }
                            Keycode::End => {
                                let text = data.text.get();
                                let len = text.chars().count();
                                if self.cursor_pos_char + 1 <= len {
                                    self.move_cursor(
                                        &text,
                                        len,
                                        control_context.get_rect(),
                                        drawing_context.get_resources(),
                                    );
                                }
                            }
                            Keycode::Left => {
                                let text = data.text.get();
                                if self.cursor_pos_char > 0 {
                                    self.move_cursor(
                                        &text,
                                        self.cursor_pos_char - 1,
                                        control_context.get_rect(),
                                        drawing_context.get_resources(),
                                    );
                                }
                            }
                            Keycode::Right => {
                                let text = data.text.get();
                                if self.cursor_pos_char + 1 <= text.chars().count() {
                                    self.move_cursor(
                                        &text,
                                        self.cursor_pos_char + 1,
                                        control_context.get_rect(),
                                        drawing_context.get_resources(),
                                    );
                                }
                            }
                            _ => (),
                        }
                    }

                    if let Some(ref text) = key_event.text {
                        self.insert_str(
                            data,
                            &text,
                            control_context.get_rect(),
                            drawing_context.get_resources(),
                        );
                    }

                    control_context.set_is_dirty(true);
                }
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        data: &mut TextBox,
        _control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) -> Size {
        let (_text_width, text_height) = drawing_context
            .get_resources()
            .get_font_dimensions(self.font_name, self.font_size, &data.text.get())
            .unwrap_or((0, 0));

        let width = if size.width.is_infinite() {
            8.0f32 + 8.0f32
        } else {
            size.width.max(8.0f32 + 8.0f32)
        };

        Size::new(width, text_height as f32 + 8.0f32)
    }

    fn set_rect(
        &mut self,
        _data: &mut TextBox,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        rect: Rect,
    ) {
        self.update_offset_x(rect);
    }

    fn hit_test(
        &self,
        _data: &TextBox,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn to_primitives(
        &self,
        data: &TextBox,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();

        let rect = control_context.get_rect();
        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        let (text_width, text_height) = drawing_context
            .get_resources()
            .get_font_dimensions(self.font_name, self.font_size, &data.text.get())
            .unwrap_or((0, 0));

        default_theme::border_3d_edit(
            &mut vec,
            x,
            y,
            width,
            height,
            self.is_hover,
            self.is_focused,
        );

        gradient_rect(
            &mut vec,
            x + 3.0f32,
            y + 3.0f32,
            width - 6.0f32,
            height - 6.0f32,
            if self.is_focused {
                [1.0, 1.0, 1.0, 0.75]
            } else if self.is_hover {
                [1.0, 1.0, 1.0, 0.675]
            } else {
                [1.0, 1.0, 1.0, 0.6]
            },
            if self.is_focused {
                [0.9, 0.9, 0.9, 0.75]
            } else if self.is_hover {
                [0.9, 0.9, 0.9, 0.675]
            } else {
                [0.9, 0.9, 0.9, 0.6]
            },
        );

        let mut vec2 = Vec::new();

        vec2.push(Primitive::Text {
            resource_key: self.font_name.to_string(),
            color: [0.0, 0.0, 0.0, 1.0],
            position: PixelPoint::new(x + 4.0f32, y + (height - text_height as f32) / 2.0),
            clipping_rect: PixelRect::new(
                PixelPoint::new(x + 4.0f32, y + 4.0f32),
                PixelSize::new(text_width as f32, height),
            ),
            size: Length::new(self.font_size as f32),
            text: data.text.get(),
        });

        // draw cursor
        if self.is_focused {
            vec2.push(Primitive::Rectangle {
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

        if self.offset_x != 0.0f32 {
            vec2.translate(PixelPoint::new(-self.offset_x, 0.0f32));
        }

        vec2 = vec2.clip(PixelRect::new(
            PixelPoint::new(x + 4.0f32, y + 4.0f32),
            PixelSize::new(width - 8.0f32, height - 8.0f32),
        ));

        vec.append(&mut vec2);

        (vec, Vec::new())
    }
}
