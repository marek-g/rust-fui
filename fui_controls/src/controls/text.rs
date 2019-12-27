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

impl View for Text {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        Control::new(self, TextDefaultStyle::new(), context)
    }
}

//
// Text Default Style
//

pub struct TextDefaultStyle {
    rect: Rect,
    event_subscriptions: Vec<EventSubscription>,
    font_name: &'static str,
    font_size: u8,
}

impl TextDefaultStyle {
    pub fn new() -> TextDefaultStyle {
        TextDefaultStyle {
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

impl Style<Text> for TextDefaultStyle {
    fn setup_dirty_watching(&mut self, data: &mut Text, control: &Rc<RefCell<Control<Text>>>) {
        self.event_subscriptions
            .push(data.text.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        _data: &mut Text,
        _children: &Box<dyn ChildrenSource>,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Text,
        _children: &Box<dyn ChildrenSource>,
        resources: &mut dyn Resources,
        _size: Size,
    ) {
        let (text_width, text_height) = resources
            .get_font_dimensions(self.font_name, self.font_size, &data.text.get())
            .unwrap_or((0, 0));
        self.rect = Rect::new(0.0f32, 0.0f32, text_width as f32, text_height as f32)
    }

    fn set_rect(&mut self, _data: &mut Text, _children: &Box<dyn ChildrenSource>, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &Text,
        _children: &Box<dyn ChildrenSource>,
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
        data: &Text,
        _children: &Box<dyn ChildrenSource>,
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

        vec.push(Primitive::Text {
            resource_key: self.font_name.to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            position: PixelPoint::new(
                x + (width - text_width as f32) / 2.0,
                y + (height - text_height as f32) / 2.0,
            ),
            clipping_rect: PixelRect::new(PixelPoint::new(x, y), PixelSize::new(width, height)),
            size: Length::new(self.font_size as f32),
            text: data.text.get(),
        });

        vec
    }
}
