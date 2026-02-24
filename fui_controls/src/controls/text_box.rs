use std::cell::RefCell;
use std::rc::Rc;

use crate::style::default_theme::gradient_rect;
use crate::utils::text_buffer::TextBuffer;
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

    buffer: TextBuffer,

    is_hover: bool,
    is_focused: bool,

    cursor_pos_px: f32,
    offset_x: f32,

    selection_start_px: f32,

    paragraph: Option<DrawingParagraph>,
}

impl DefaultTextBoxStyle {
    pub fn new(params: DefaultTextBoxStyleParams) -> Self {
        DefaultTextBoxStyle {
            params,

            buffer: TextBuffer::new(String::new()),

            is_hover: false,
            is_focused: false,

            cursor_pos_px: 0.0f32,
            offset_x: 0.0f32,

            selection_start_px: 0.0,

            paragraph: None,
        }
    }

    fn sync_with_buffer(&mut self, fonts: &DrawingFonts, rect: Rect) {
        self.update_paragraph(fonts);

        // calc cursor px and selection px
        self.cursor_pos_px = self.calc_px_from_char(self.buffer.get_cursor());
        let (s, _) = self
            .buffer
            .get_selection()
            .unwrap_or((self.buffer.get_cursor(), 0));
        self.selection_start_px = self.calc_px_from_char(s);

        // scroll to cursor if needed
        self.update_offset_x(rect);
    }

    fn update_paragraph(&mut self, fonts: &DrawingFonts) {
        let display_text = self.get_display_text();

        let mut builder = DrawingParagraphBuilder::new(fonts).unwrap();
        builder.push_style(ParagraphStyle::simple(
            self.params.font_name,
            self.params.font_size,
            Color::rgb(0.0, 0.0, 0.0),
        ));
        builder.add_text(&display_text);
        self.paragraph = Some(builder.build(f32::INFINITY).unwrap());
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

    fn calc_px_from_char(&self, char_idx: usize) -> f32 {
        if char_idx == 0 {
            return 0.0;
        }

        let p = self.paragraph.as_ref().unwrap();
        let info = p.create_glyph_info_at_code_unit_index_utf16(char_idx.saturating_sub(1));
        if let Some(gi) = info {
            let bounds = gi.get_grapheme_cluster_bounds();
            if char_idx >= self.buffer.get_text().chars().count() {
                bounds.origin.x as f32 + bounds.size.width as f32
            } else {
                let current = p.create_glyph_info_at_code_unit_index_utf16(char_idx);
                current
                    .map(|c| c.get_grapheme_cluster_bounds().origin.x as f32)
                    .unwrap_or(0.0)
            }
        } else {
            0.0
        }
    }

    fn calc_cursor_pos(&mut self, pos: &Point, rect: Rect, fonts: &DrawingFonts) -> usize {
        self.update_paragraph(&fonts);

        let paragraph = self.paragraph.as_ref().unwrap();

        let pos = (pos.x - rect.x - 4.0f32) + self.offset_x;
        let glyph_info = paragraph.create_glyph_info_at_paragraph_coordinates(pos as f64, 0.0);
        if let Some(glyph_info) = glyph_info {
            let rect = glyph_info.get_grapheme_cluster_bounds();
            if pos >= rect.origin.x + rect.size.width {
                glyph_info.get_grapheme_cluster_code_unit_range_end_utf16()
            } else {
                glyph_info.get_grapheme_cluster_code_unit_range_begin_utf16()
            }
        } else {
            0
        }
    }

    fn get_display_text(&self) -> String {
        if self.params.password {
            self.buffer.get_text().chars().map(|_| '*').collect()
        } else {
            self.buffer.get_text().to_string()
        }
    }
}

impl Style<TextBox> for DefaultTextBoxStyle {
    fn setup(&mut self, data: &mut TextBox, control_context: &mut ControlContext) {
        self.buffer.set_text(data.text.get());
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
                let rect = control_context.get_rect();
                let cursor_pos = self.calc_cursor_pos(position, rect, drawing_context.fonts);

                //let shift_pressed = drawing_context.modifiers.shift;
                let shift_pressed = false;
                self.buffer.set_cursor(cursor_pos, shift_pressed);
                self.sync_with_buffer(drawing_context.fonts, rect);
                control_context.set_is_dirty(true);
            }

            ControlEvent::KeyboardInput(ref key_event) if key_event.state == KeyState::Pressed => {
                let mut changed = false;
                let ctrl = key_event.modifiers.ctrl;
                let shift = key_event.modifiers.shift;

                let mut handled = false;
                if let Some(ref key_code) = key_event.keycode {
                    match key_code {
                        Keycode::Backspace => {
                            self.buffer.backspace();
                            changed = true;
                            handled = true;
                        }
                        Keycode::Delete => {
                            self.buffer.delete();
                            changed = true;
                            handled = true;
                        }
                        Keycode::Home => {
                            self.buffer.set_cursor(0, shift);
                            changed = true;
                            handled = true;
                        }
                        Keycode::End => {
                            self.buffer
                                .set_cursor(self.buffer.get_text().chars().count(), shift);
                            changed = true;
                            handled = true;
                        }
                        Keycode::Left => {
                            self.buffer
                                .set_cursor(self.buffer.get_cursor().saturating_sub(1), shift);
                            changed = true;
                            handled = true;
                        }
                        Keycode::Right => {
                            self.buffer.set_cursor(self.buffer.get_cursor() + 1, shift);
                            changed = true;
                            handled = true;
                        }
                        Keycode::Esc | Keycode::Tab | Keycode::Enter => {
                            handled = true;
                        }
                        _ => {}
                    }
                }

                if !handled {
                    if let Some(ref t) = key_event.text {
                        self.buffer.insert_str(t);
                        changed = true;
                    }
                }

                if changed {
                    data.text.set(self.buffer.get_text().to_string());
                    self.sync_with_buffer(drawing_context.fonts, control_context.get_rect());
                    control_context.set_is_dirty(true);
                }
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut TextBox,
        _control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        self.update_paragraph(&drawing_context.fonts);

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

            if clip {
                drawing_context.display.restore();
            }
        }
    }
}
