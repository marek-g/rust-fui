use std::cell::RefCell;
use std::rc::Rc;

use common::*;
use control::*;
use control_object::*;
use drawing::primitive::Primitive;
use drawing::units::{UserPixelPoint, UserPixelRect, UserPixelSize, UserPixelThickness};
use drawing_context::DrawingContext;
use events::*;
use observable::*;
use Property;
use style::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct TextProperties {
    pub text: Property<String>,
}

pub struct Text {
    pub properties: TextProperties,
}

impl Text {
    pub fn new(properties: TextProperties) -> Self {
        Text {
            properties: properties,
        }
    }
}

impl ControlBehaviour for Control<Text> {
    fn handle_event(&mut self, _event: ControlEvent) {}
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
            .push(data.properties.text.dirty_watching(control));
    }

    fn get_preferred_size(
        &self,
        data: &Text,
        children: &Vec<Rc<RefCell<ControlObject>>>,
        drawing_context: &mut DrawingContext,
        _size: Size,
    ) -> Size {
        let (text_width, text_height) = drawing_context
            .get_font_dimensions(self.font_name, self.font_size, &data.properties.text.get())
            .unwrap_or((0, 0));
        Size::new(text_width as f32, text_height as f32)
    }

    fn set_rect(&mut self, _data: &Text, children: &Vec<Rc<RefCell<ControlObject>>>, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Text, children: &Vec<Rc<RefCell<ControlObject>>>, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(&self, data: &Text, children: &Vec<Rc<RefCell<ControlObject>>>, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let (text_width, text_height) = drawing_context
            .get_font_dimensions(self.font_name, self.font_size, &data.properties.text.get())
            .unwrap_or((0, 0));

        vec.push(Primitive::Text {
            resource_key: self.font_name.to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            position: UserPixelPoint::new(
                x + (width - text_width as f32) / 2.0,
                y + (height - text_height as f32) / 2.0,
            ),
            size: self.font_size as u16,
            text: data.properties.text.get(),
        });

        vec
    }
}
