use std::cell::RefCell;
use std::rc::Rc;

use crate::style::default_theme::gradient_rect;
use fui_core::*;
use fui_drawing::prelude::*;
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
    ) -> Rc<RefCell<dyn ControlObject>> {
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
pub struct DefaultTextBoxStyleParams {
    #[builder(default = false)]
    pub password: bool,
    #[builder(default = "sans-serif")]
    font_name: &'static str,
    #[builder(default = 20f32)]
    font_size: f32,
}

pub struct DefaultTextBoxStyle {
    params: DefaultTextBoxStyleParams,

    is_hover: bool,
    is_focused: bool,

    cursor_pos_char: usize,
    cursor_pos_px: f32,
    offset_x: f32,

    paragraph: Option<DrawingParagraph>,
}

impl DefaultTextBoxStyle {
    pub fn new(params: DefaultTextBoxStyleParams) -> Self {
        DefaultTextBoxStyle {
            params,

            is_hover: false,
            is_focused: false,

            cursor_pos_char: 0,
            cursor_pos_px: 0.0f32,
            offset_x: 0.0f32,

            paragraph: None,
        }
    }

    fn update_paragraph(&mut self, text: &str, fonts: &DrawingFonts, recreate: bool) {
        if !recreate && self.paragraph.is_some() {
            return;
        }

        let display_text = self.get_display_text(text);

        let mut builder = DrawingParagraphBuilder::new(fonts).unwrap();
        builder.push_style(ParagraphStyle::simple(
            self.params.font_name,
            self.params.font_size,
            Color::rgb(0.0, 0.0, 0.0),
        ));
        builder.add_text(&display_text);
        let paragraph = builder.build().unwrap();

        self.paragraph = Some(paragraph);
    }

    fn calc_cursor_pos(
        &mut self,
        text: &str,
        pos: &Point,
        rect: Rect,
        fonts: &DrawingFonts,
    ) -> (usize, f32) {
        self.update_paragraph(&text, &fonts, false);

        let paragraph = self.paragraph.as_ref().unwrap();

        let pos = (pos.x - rect.x - 4.0f32) + self.offset_x;
        let glyph_info = paragraph.create_glyph_info_at_paragraph_coordinates(pos as f64, 0.0);
        if let Some(glyph_info) = glyph_info {
            let rect = glyph_info.get_grapheme_cluster_bounds();
            let begin = glyph_info.get_grapheme_cluster_code_unit_range_begin_utf16();
            (begin, rect.origin.x)
        } else {
            (0, 0.0)
        }
    }

    fn calc_cursor_pos_px(
        &mut self,
        text: &str,
        cursor_pos_char: usize,
        fonts: &DrawingFonts,
    ) -> f32 {
        self.update_paragraph(&text, &fonts, false);

        let paragraph = self.paragraph.as_ref().unwrap();
        let glyph_info = paragraph.create_glyph_info_at_code_unit_index_utf16(cursor_pos_char);
        if let Some(glyph_info) = glyph_info {
            glyph_info.get_grapheme_cluster_bounds().origin.x
        } else {
            0.0
        }
    }

    fn move_cursor(
        &mut self,
        text: &str,
        cursor_pos_char: usize,
        rect: Rect,
        fonts: &DrawingFonts,
    ) {
        self.update_paragraph(&text, &fonts, false);

        let cursor_pos_px = self.calc_cursor_pos_px(&text, cursor_pos_char, fonts);
        self.cursor_pos_char = cursor_pos_char;
        self.cursor_pos_px = cursor_pos_px;

        self.update_offset_x(rect);
    }

    fn insert_str(&mut self, data: &mut TextBox, text: &str, rect: Rect, fonts: &DrawingFonts) {
        let t = data.text.get();
        let t: String = t
            .chars()
            .take(self.cursor_pos_char)
            .chain(text.chars())
            .chain(t.chars().skip(self.cursor_pos_char))
            .collect();

        self.update_paragraph(&t, &fonts, true);

        let new_cursor_pos_char = self.cursor_pos_char + text.chars().count();

        self.move_cursor(&t, new_cursor_pos_char, rect, fonts);
        data.text.set(t);
    }

    fn remove_char(&mut self, data: &mut TextBox, pos: usize, rect: Rect, fonts: &DrawingFonts) {
        let t = data.text.get();
        let t: String = t.chars().take(pos).chain(t.chars().skip(pos + 1)).collect();

        self.update_paragraph(&t, &fonts, true);

        if pos < self.cursor_pos_char {
            self.move_cursor(&t, self.cursor_pos_char - 1, rect, fonts);
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

    fn get_display_text(&self, text: &str) -> String {
        if self.params.password {
            text.chars().map(|_| '*').collect()
        } else {
            text.to_string()
        }
    }
}

impl Style<TextBox> for DefaultTextBoxStyle {
    fn setup(&mut self, data: &mut TextBox, control_context: &mut ControlContext) {
        control_context.dirty_watch_property(&data.text);
    }

    fn handle_event(
        &mut self,
        data: &mut TextBox,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
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
                    drawing_context.fonts,
                );
                self.cursor_pos_char = cursor_pos.0;
                self.cursor_pos_px = cursor_pos.1;
                control_context.set_is_dirty(true);
            }

            ControlEvent::KeyboardInput(ref key_event) => {
                let mut handled = false;
                if key_event.state == KeyState::Pressed {
                    if let Some(ref key_code) = key_event.keycode {
                        match key_code {
                            Keycode::Backspace => {
                                if self.cursor_pos_char > 0 {
                                    self.remove_char(
                                        data,
                                        self.cursor_pos_char - 1,
                                        control_context.get_rect(),
                                        drawing_context.fonts,
                                    );
                                }
                                handled = true;
                            }
                            Keycode::Delete => {
                                let text = data.text.get();
                                if self.cursor_pos_char < text.chars().count() {
                                    self.remove_char(
                                        data,
                                        self.cursor_pos_char,
                                        control_context.get_rect(),
                                        drawing_context.fonts,
                                    );
                                }
                                handled = true;
                            }
                            Keycode::Home => {
                                if self.cursor_pos_char > 0 {
                                    self.move_cursor(
                                        &data.text.get(),
                                        0,
                                        control_context.get_rect(),
                                        drawing_context.fonts,
                                    );
                                }
                                handled = true;
                            }
                            Keycode::End => {
                                let text = self.get_display_text(&data.text.get());
                                let len = text.chars().count();
                                if self.cursor_pos_char + 1 <= len {
                                    self.move_cursor(
                                        &data.text.get(),
                                        len,
                                        control_context.get_rect(),
                                        drawing_context.fonts,
                                    );
                                }
                                handled = true;
                            }
                            Keycode::Left => {
                                if self.cursor_pos_char > 0 {
                                    self.move_cursor(
                                        &data.text.get(),
                                        self.cursor_pos_char - 1,
                                        control_context.get_rect(),
                                        drawing_context.fonts,
                                    );
                                }
                                handled = true;
                            }
                            Keycode::Right => {
                                let text = self.get_display_text(&data.text.get());
                                if self.cursor_pos_char + 1 <= text.chars().count() {
                                    self.move_cursor(
                                        &data.text.get(),
                                        self.cursor_pos_char + 1,
                                        control_context.get_rect(),
                                        drawing_context.fonts,
                                    );
                                }
                                handled = true;
                            }
                            Keycode::Esc | Keycode::Tab | Keycode::Enter => {
                                handled = true;
                            }
                            _ => (),
                        }
                    }

                    if !handled {
                        if let Some(ref text) = key_event.text {
                            self.insert_str(
                                data,
                                &text,
                                control_context.get_rect(),
                                drawing_context.fonts,
                            );
                        }
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
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        self.update_paragraph(&data.text.get(), &drawing_context.fonts, false);

        let paragraph = self.paragraph.as_ref().unwrap();
        let paragraph_height = paragraph.get_height();

        let width = if size.width.is_infinite() {
            8.0f32 + 8.0f32
        } else {
            size.width.max(8.0f32 + 8.0f32)
        };

        Size::new(width, paragraph_height + 8.0f32)
    }

    fn set_rect(
        &mut self,
        _data: &mut TextBox,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
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

    fn draw(
        &mut self,
        _data: &TextBox,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        if let Some(paragraph) = &self.paragraph {
            let r = control_context.get_rect();
            let x = r.x;
            let y = r.y;
            let width = r.width;
            let height = r.height;

            let text_width = paragraph.get_longest_line_width();
            let text_height = paragraph.get_height();

            default_theme::border_3d_edit(
                &mut drawing_context.display,
                x,
                y,
                width,
                height,
                self.is_hover,
                self.is_focused,
            );

            gradient_rect(
                &mut drawing_context.display,
                x + 3.0f32,
                y + 3.0f32,
                width - 6.0f32,
                height - 6.0f32,
                if self.is_focused {
                    Color::rgba(1.0, 1.0, 1.0, 0.75)
                } else if self.is_hover {
                    Color::rgba(1.0, 1.0, 1.0, 0.675)
                } else {
                    Color::rgba(1.0, 1.0, 1.0, 0.6)
                },
                if self.is_focused {
                    Color::rgba(0.9, 0.9, 0.9, 0.75)
                } else if self.is_hover {
                    Color::rgba(0.9, 0.9, 0.9, 0.675)
                } else {
                    Color::rgba(0.9, 0.9, 0.9, 0.6)
                },
            );

            let clip = text_width > width - 8.0 || text_height > height - 8.0;

            if clip {
                drawing_context.display.save();
                drawing_context.display.clip_rect(
                    rect(x + 4.0f32, y + 4.0f32, width - 8.0f32, height - 8.0f32),
                    ClipOperation::Intersect,
                );
                if self.offset_x != 0.0f32 {
                    drawing_context.display.translate(-self.offset_x, 0.0f32);
                }
            }

            drawing_context.display.draw_paragraph(
                (x + 4.0f32, y + (height - text_height as f32) / 2.0),
                &paragraph,
            );

            if clip {
                drawing_context.display.restore();
            }

            // draw cursor
            if self.is_focused {
                drawing_context.display.draw_rect(
                    rect(
                        x + 4.0f32 + self.cursor_pos_px,
                        y + (height - text_height as f32) / 2.0,
                        2.0f32,
                        text_height as f32,
                    ),
                    Color::rgb(1.0, 1.0, 0.0),
                );
            }
        }
    }
}
