use std::rc::Rc;

use fui_core::*;
use fui_drawing::prelude::*;
use typed_builder::TypedBuilder;

use crate::style::default_theme;
use crate::style::{FontFamily, FontSize, Foreground};

#[derive(TypedBuilder)]
pub struct Text {
    pub text: Property<String>,
}

impl Text {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| Box::new(DefaultTextStyle::new())),
            context,
        )
    }
}

//
// Default Text Style
//

pub struct DefaultTextStyle {
    paragraph: Option<DrawingParagraph>,
}

impl DefaultTextStyle {
    pub fn new() -> Self {
        DefaultTextStyle { paragraph: None }
    }
}

impl Style<Text> for DefaultTextStyle {
    fn setup(&mut self, data: &mut Text, control_context: &ControlContext) {
        control_context.dirty_watch_property(&data.text);
    }

    fn handle_event(
        &mut self,
        _data: &mut Text,
        _control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Text,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        _size: Size,
    ) -> Size {
        // Get values from attached inherited values, fall back to defaults
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
            .unwrap_or(default_theme::DEFAULT_FOREGROUND.into());

        let mut builder = DrawingParagraphBuilder::new(drawing_context.fonts).unwrap();
        builder.push_style(ParagraphStyle::simple(&font_family, font_size, &foreground));
        builder.add_text(&data.text.get());

        let paragraph = builder.build(f32::INFINITY).unwrap();
        let paragraph_width = paragraph.get_longest_line_width();
        let paragraph_height = paragraph.get_height();

        self.paragraph = Some(paragraph);

        Size::new(paragraph_width, paragraph_height)
    }

    fn set_rect(
        &mut self,
        _data: &mut Text,
        _control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _rect: Rect,
    ) {
    }

    fn hit_test(
        &self,
        _data: &Text,
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
        _data: &Text,
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

            let clip = text_width > width || text_height > height;

            if clip {
                drawing_context.display.save();
                drawing_context
                    .display
                    .clip_rect(rect(x, y, width, height), ClipOperation::Intersect);
            }

            drawing_context.display.draw_paragraph(
                (
                    x + (width - text_width as f32) / 2.0,
                    y + (height - text_height as f32) / 2.0,
                ),
                paragraph,
            );

            if clip {
                drawing_context.display.restore();
            }
        }
    }
}
