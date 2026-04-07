use std::rc::Rc;

use crate::style::default_theme::gradient_rect;
use crate::utils::text_buffer::TextBuffer;
use fui_core::*;
use fui_drawing::prelude::*;
use typed_builder::TypedBuilder;

use crate::style::default_theme;
use crate::style::{FontFamily, FontSize, Foreground};

#[derive(TypedBuilder)]
pub struct TextBox {
    pub text: Property<String>,
}

impl TextBox {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject> {
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
}

pub struct DefaultTextBoxStyle {
    params: DefaultTextBoxStyleParams,

    buffer: Option<TextBuffer>,

    is_hover: bool,
    is_focused: bool,

    cursor_pos_px: f32,
    offset_x: f32,

    selection_start_px: f32,

    paragraph: Option<DrawingParagraph>,
}

impl DefaultTextBoxStyle {
    pub fn new(_params: DefaultTextBoxStyleParams) -> Self {
        DefaultTextBoxStyle {
            params: _params,
            buffer: None,

            is_hover: false,
            is_focused: false,

            cursor_pos_px: 0.0f32,
            offset_x: 0.0f32,

            selection_start_px: 0.0,

            paragraph: None,
        }
    }

    fn buf(&self) -> &TextBuffer {
        self.buffer
            .as_ref()
            .expect("Buffer should be initialized in setup")
    }

    fn buf_mut(&mut self) -> &mut TextBuffer {
        self.buffer
            .as_mut()
            .expect("Buffer should be initialized in setup")
    }

    fn sync_with_buffer(
        &mut self,
        fonts: &DrawingFonts,
        rect: Rect,
        control_context: &ControlContext,
    ) {
        self.update_paragraph(fonts, control_context);

        // calc cursor px and selection px
        self.cursor_pos_px = self.calc_px_from_char(self.buf().get_cursor());
        self.selection_start_px = self.calc_px_from_char(self.buf().get_selection_start());

        // scroll to cursor if needed
        self.update_offset_x(rect);
    }

    fn update_paragraph(&mut self, fonts: &DrawingFonts, control_context: &ControlContext) {
        let display_text = self.get_display_text();

        // Get values from inherited attached values, fall back to defaults
        let font_family = control_context
            .get_inherited_value::<FontFamily>()
            .map(|p| p.get())
            .unwrap_or_else(|| default_theme::DEFAULT_FONT_FAMILY.to_string());

        let font_size = control_context
            .get_inherited_value::<FontSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::DEFAULT_FONT_SIZE.into());

        let foreground = control_context
            .get_inherited_value::<Foreground>()
            .map(|p| p.get())
            .unwrap_or(default_theme::DEFAULT_EDIT_TEXT_COLOR.into());

        let mut builder = DrawingParagraphBuilder::new(fonts).unwrap();
        builder.push_style(ParagraphStyle::simple(&font_family, font_size, &foreground));
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
            let text_len = self.buf().get_text_property().read().chars().count();

            if char_idx >= text_len {
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

    fn calc_cursor_pos(
        &mut self,
        pos: &Point,
        rect: Rect,
        fonts: &DrawingFonts,
        control_context: &ControlContext,
    ) -> usize {
        self.update_paragraph(fonts, control_context);

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
        let text_prop = self.buf().get_text_property();
        let text_lock = text_prop.read();

        if self.params.password {
            text_lock.chars().map(|_| '*').collect()
        } else {
            text_lock.clone()
        }
    }

    fn copy_to_clipboard(&self, control_context: &ControlContext) {
        if let Some(selected) = self.buf().get_selected_string() {
            if let Some(clipboard) = control_context
                .get_services()
                .as_ref()
                .map(|s| s.get_clipboard_service())
            {
                clipboard.set_text(&selected, ClipboardMode::Clipboard);
            }
        }
    }

    fn paste_from_clipboard(&mut self, control_context: &ControlContext) -> bool {
        if let Some(clipboard) = control_context
            .get_services()
            .as_ref()
            .map(|s| s.get_clipboard_service())
        {
            if let Some(text) = clipboard.get_text(ClipboardMode::Clipboard) {
                self.buf_mut().insert_str(&text);
                return true;
            }
        }
        false
    }

    fn cut_to_clipboard(&mut self, control_context: &ControlContext) -> bool {
        if let Some(selected) = self.buf_mut().delete_selected_text() {
            if let Some(clipboard) = control_context
                .get_services()
                .as_ref()
                .map(|s| s.get_clipboard_service())
            {
                clipboard.set_text(&selected, ClipboardMode::Clipboard);
            }
            return true;
        }
        false
    }
}

impl Style<TextBox> for DefaultTextBoxStyle {
    fn setup(&mut self, data: &mut TextBox, control_context: &ControlContext) {
        self.buffer = Some(TextBuffer::new(data.text.clone()));
        control_context.dirty_watch_property(&data.text);
    }

    fn handle_event(
        &mut self,
        _data: &mut TextBox,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        if self.buffer.is_none() {
            return;
        }

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
                let cursor_pos =
                    self.calc_cursor_pos(position, rect, drawing_context.fonts, control_context);

                let shift_pressed = false;
                self.buf_mut().set_cursor(cursor_pos, shift_pressed);
                self.sync_with_buffer(drawing_context.fonts, rect, control_context);
                control_context.set_is_dirty(true);
            }

            ControlEvent::KeyboardInput(ref key_event) if key_event.state == KeyState::Pressed => {
                let mut changed = false;
                let shift = key_event.modifiers.shift;
                let ctrl = key_event.modifiers.ctrl;
                let mut handled = false;

                if let Some(ref key_code) = key_event.keycode {
                    match key_code {
                        // Edition
                        Keycode::Backspace => {
                            self.buf_mut().backspace();
                            changed = true;
                            handled = true;
                        }
                        Keycode::Delete => {
                            if shift {
                                changed = self.cut_to_clipboard(control_context);
                            } else {
                                self.buf_mut().delete();
                                changed = true;
                            }
                            handled = true;
                        }

                        // Navigation & selection
                        Keycode::Home => {
                            self.buf_mut().set_cursor(0, shift);
                            changed = true;
                            handled = true;
                        }
                        Keycode::End => {
                            let len = self.buf().get_text_property().read().chars().count();
                            self.buf_mut().set_cursor(len, shift);
                            changed = true;
                            handled = true;
                        }
                        Keycode::Left => {
                            if ctrl {
                                self.buf_mut().move_word_left(shift);
                            } else {
                                let cur = self.buf().get_cursor();
                                self.buf_mut().set_cursor(cur.saturating_sub(1), shift);
                            }
                            changed = true;
                            handled = true;
                        }
                        Keycode::Right => {
                            if ctrl {
                                self.buf_mut().move_word_right(shift);
                            } else {
                                let cur = self.buf().get_cursor();
                                self.buf_mut().set_cursor(cur + 1, shift);
                            }
                            changed = true;
                            handled = true;
                        }
                        Keycode::KeyA if ctrl => {
                            self.buf_mut().select_all();
                            changed = true;
                            handled = true;
                        }

                        // Clipboard
                        Keycode::KeyC if ctrl => {
                            self.copy_to_clipboard(control_context);
                            handled = true;
                        }
                        Keycode::Insert if ctrl => {
                            self.copy_to_clipboard(control_context);
                            handled = true;
                        }
                        Keycode::KeyV if ctrl => {
                            changed = self.paste_from_clipboard(control_context);
                            handled = true;
                        }
                        Keycode::Insert if shift => {
                            changed = self.paste_from_clipboard(control_context);
                            handled = true;
                        }
                        Keycode::KeyX if ctrl => {
                            changed = self.cut_to_clipboard(control_context);
                            handled = true;
                        }

                        // Ignored keys
                        Keycode::Esc | Keycode::Tab | Keycode::Enter => {
                            handled = true;
                        }
                        _ => {}
                    }
                }

                if !handled && !ctrl {
                    if let Some(ref t) = key_event.text {
                        self.buf_mut().insert_str(t);
                        changed = true;
                    }
                }

                if changed {
                    self.sync_with_buffer(
                        drawing_context.fonts,
                        control_context.get_rect(),
                        control_context,
                    );
                    control_context.set_is_dirty(true);
                }
            }
            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut TextBox,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        self.update_paragraph(&drawing_context.fonts, control_context);

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
        _control_context: &ControlContext,
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
    ) -> Option<Rc<dyn ControlObject>> {
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

            let clip =
                text_width > width - 8.0 || text_height > height - 8.0 || self.offset_x != 0.0f32;

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

            // draw selection
            if let Some((_start_idx, _end_idx)) = self.buf().get_selection() {
                // we already know these values in pixels
                let s_px = self.selection_start_px;
                let c_px = self.cursor_pos_px;

                let rect_x = s_px.min(c_px);
                let rect_width = (s_px - c_px).abs();

                drawing_context.display.draw_rect(
                    rect(
                        x + 4.0f32 + rect_x,
                        y + (height - text_height as f32) / 2.0,
                        rect_width,
                        text_height,
                    ),
                    Color::rgba(0.0, 0.47, 0.83, 0.35),
                );
            }

            // draw text
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
