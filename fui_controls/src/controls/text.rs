use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize};
use euclid::Length;
use fui_core::*;
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
    ) -> Rc<RefCell<StyledControl<Self>>> {
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
    #[builder(default = [1.0f32, 1.0f32, 1.0f32, 1.0f32])]
    pub color: Color,
}

pub struct DefaultTextStyle {
    params: DefaultTextStyleParams,
    event_subscriptions: Vec<EventSubscription>,
    font_name: &'static str,
    font_size: u8,
}

impl DefaultTextStyle {
    pub fn new(params: DefaultTextStyleParams) -> Self {
        DefaultTextStyle {
            params,
            event_subscriptions: Vec::new(),
            font_name: "OpenSans-Regular.ttf",
            font_size: 20u8,
        }
    }
}

impl Style<Text> for DefaultTextStyle {
    fn setup(&mut self, data: &mut Text, control_context: &mut ControlContext) {
        self.event_subscriptions
            .push(data.text.dirty_watching(&control_context.get_self_rc()));
    }

    fn handle_event(
        &mut self,
        _data: &mut Text,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Text,
        _control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        _size: Size,
    ) -> Size {
        let (text_width, text_height) = drawing_context
            .get_resources()
            .get_font_dimensions(self.font_name, self.font_size, &data.text.get())
            .unwrap_or((0, 0));

        Size::new(text_width as f32, text_height as f32)
    }

    fn set_rect(
        &mut self,
        _data: &mut Text,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
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

    fn to_primitives(
        &self,
        data: &Text,
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

        vec.push(Primitive::Text {
            resource_key: self.font_name.to_string(),
            color: self.params.color,
            position: PixelPoint::new(
                x + (width - text_width as f32) / 2.0,
                y + (height - text_height as f32) / 2.0,
            ),
            clipping_rect: PixelRect::new(PixelPoint::new(x, y), PixelSize::new(width, height)),
            size: Length::new(self.font_size as f32),
            text: data.text.get(),
        });

        (vec, Vec::new())
    }
}

//
// Dynamic Text Style
//

#[derive(TypedBuilder)]
pub struct DynamicTextStyleParams {
    #[builder(default = Property::new([1.0f32, 1.0f32, 1.0f32, 1.0f32]))]
    pub color: Property<Color>,
}

pub struct DynamicTextStyle {
    params: DynamicTextStyleParams,
    event_subscriptions: Vec<EventSubscription>,
    font_name: &'static str,
    font_size: u8,
    is_hover: bool,
}

impl DynamicTextStyle {
    pub fn new(params: DynamicTextStyleParams) -> Self {
        DynamicTextStyle {
            params,
            event_subscriptions: Vec::new(),
            font_name: "OpenSans-Regular.ttf",
            font_size: 20u8,
            is_hover: false,
        }
    }
}

impl Style<Text> for DynamicTextStyle {
    fn setup(&mut self, data: &mut Text, control_context: &mut ControlContext) {
        self.event_subscriptions
            .push(data.text.dirty_watching(&control_context.get_self_rc()));
        self.event_subscriptions.push(
            self.params
                .color
                .dirty_watching(&control_context.get_self_rc()),
        );
    }

    fn handle_event(
        &mut self,
        _data: &mut Text,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::HoverChange(value) => {
                self.is_hover = value;
                control_context.set_is_dirty(true);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        data: &mut Text,
        _control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        _size: Size,
    ) -> Size {
        let (text_width, text_height) = drawing_context
            .get_resources()
            .get_font_dimensions(self.font_name, self.font_size, &data.text.get())
            .unwrap_or((0, 0));

        Size::new(text_width as f32, text_height as f32)
    }

    fn set_rect(
        &mut self,
        _data: &mut Text,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
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

    fn to_primitives(
        &self,
        data: &Text,
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

        vec.push(Primitive::Text {
            resource_key: self.font_name.to_string(),
            color: self.params.color.get(),
            position: PixelPoint::new(
                x + (width - text_width as f32) / 2.0,
                y + (height - text_height as f32) / 2.0,
            ),
            clipping_rect: PixelRect::new(PixelPoint::new(x, y), PixelSize::new(width, height)),
            size: Length::new(self.font_size as f32),
            text: data.text.get(),
        });

        (vec, Vec::new())
    }
}
