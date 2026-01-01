use std::cell::RefCell;
use std::rc::Rc;

use fui_core::*;
use fui_drawing::prelude::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Text {
    pub text: Property<String>,
}

impl Text {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultTextStyle::new(
                    DefaultTextStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default Text Style
//

#[derive(TypedBuilder)]
pub struct DefaultTextStyleParams {
    #[builder(default = Color::rgba(1.0, 1.0, 1.0, 1.0))]
    pub color: Color,

    #[builder(default = "sans-serif")]
    font_name: &'static str,

    #[builder(default = 20.0)]
    font_size: f32,
}

pub struct DefaultTextStyle {
    params: DefaultTextStyleParams,

    paragraph: Option<DrawingParagraph>,
}

impl DefaultTextStyle {
    pub fn new(params: DefaultTextStyleParams) -> Self {
        DefaultTextStyle {
            params,
            paragraph: None,
        }
    }
}

impl Style<Text> for DefaultTextStyle {
    fn setup(&mut self, data: &mut Text, control_context: &mut ControlContext) {
        control_context.dirty_watch_property(&data.text);
    }

    fn handle_event(
        &mut self,
        _data: &mut Text,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Text,
        _control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        _size: Size,
    ) -> Size {
        let mut builder = DrawingParagraphBuilder::new(drawing_context.fonts).unwrap();
        builder.push_style(ParagraphStyle::simple(
            self.params.font_name,
            self.params.font_size,
            &self.params.color,
        ));
        builder.add_text(&data.text.get());

        let paragraph = builder.build().unwrap();
        let paragraph_width = paragraph.get_max_width();
        let paragraph_height = paragraph.get_height();

        self.paragraph = Some(paragraph);

        Size::new(paragraph_width, paragraph_height)
    }

    fn set_rect(
        &mut self,
        _data: &mut Text,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _rect: Rect,
    ) {
    }

    fn hit_test(
        &self,
        _data: &Text,
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

            let text_width = paragraph.get_max_width();
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

//
// Dynamic Text Style
//

#[derive(TypedBuilder)]
pub struct DynamicTextStyleParams {
    #[builder(default = Property::new([1.0f32, 1.0f32, 1.0f32, 1.0f32]))]
    pub color: Property<Color>,

    #[builder(default = Property::new("sans-serif"))]
    pub font_name: Property<String>,

    #[builder(default = Property::new(20.0))]
    pub font_size: Property<f32>,
}

pub struct DynamicTextStyle {
    params: DynamicTextStyleParams,

    paragraph: Option<DrawingParagraph>,
}

impl DynamicTextStyle {
    pub fn new(params: DynamicTextStyleParams) -> Self {
        DynamicTextStyle {
            params,
            paragraph: None,
        }
    }
}

impl Style<Text> for DynamicTextStyle {
    fn setup(&mut self, data: &mut Text, control_context: &mut ControlContext) {
        control_context.dirty_watch_property(&data.text);
        control_context.dirty_watch_property(&self.params.color);
    }

    fn handle_event(
        &mut self,
        _data: &mut Text,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Text,
        _control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        _size: Size,
    ) -> Size {
        let mut builder = DrawingParagraphBuilder::new(drawing_context.fonts).unwrap();
        builder.push_style(ParagraphStyle::simple(
            self.params.font_name.get(),
            self.params.font_size.get(),
            self.params.color.get(),
        ));
        builder.add_text(&data.text.get());

        let paragraph = builder.build().unwrap();
        let paragraph_width = paragraph.get_max_width();
        let paragraph_height = paragraph.get_height();

        self.paragraph = Some(paragraph);

        Size::new(paragraph_width, paragraph_height)
    }

    fn set_rect(
        &mut self,
        _data: &mut Text,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _rect: Rect,
    ) {
    }

    fn hit_test(
        &self,
        _data: &Text,
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

            let text_width = paragraph.get_max_width();
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
