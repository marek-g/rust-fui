use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use euclid::Length;
use fui::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct Text {
    pub text: Property<String>,
}

impl Text {
    pub fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(self,
            style.unwrap_or_else(|| {
                Box::new(DefaultTextStyle::new(DefaultTextStyleParams::builder().build()))
            }),
            context)
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
    rect: Rect,
    params: DefaultTextStyleParams,
    event_subscriptions: Vec<EventSubscription>,
    font_name: &'static str,
    font_size: u8,
}

impl DefaultTextStyle {
    pub fn new(params: DefaultTextStyleParams) -> Self {
        DefaultTextStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            params,
            event_subscriptions: Vec::new(),
            font_name: "OpenSans-Regular.ttf",
            font_size: 20u8,
        }
    }
}

impl Style<Text> for DefaultTextStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut Text,
        control: &Rc<RefCell<StyledControl<Text>>>,
    ) {
        self.event_subscriptions
            .push(data.text.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        _data: &mut Text,
        _context: &mut ControlContext,
        _resources: &mut dyn Resources,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Text,
        _context: &mut ControlContext,
        resources: &mut dyn Resources,
        _size: Size,
    ) {
        let (text_width, text_height) = resources
            .get_font_dimensions(self.font_name, self.font_size, &data.text.get())
            .unwrap_or((0, 0));
        self.rect = Rect::new(0.0f32, 0.0f32, text_width as f32, text_height as f32)
    }

    fn set_rect(&mut self, _data: &mut Text, _context: &mut ControlContext, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self, _context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Text, _context: &ControlContext, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &Text,
        _context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let (text_width, text_height) = resources
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
