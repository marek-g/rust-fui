use std::cell::{ RefCell, RefMut };
use std::rc::{ Rc, Weak };

use control::*;
use control_object::*;
use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use events::*;
use Property;

pub struct TextProperties {
    pub text: Property<String>,
}

pub struct Text {
    pub properties: TextProperties,
}

impl Text {
    pub fn new(text: String) -> Self {
        Text {
            properties: TextProperties { text: Property::new(text) },
        }
    }
}

impl ControlBehaviour for Control<Text> {
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        Vec::new()
    }

    fn handle_event(&mut self, _event: ControlEvent) {
    }
}


//
// Text Default Style
//

pub struct TextDefaultStyle {
    rect: Rect,
    font_name: &'static str,
    font_size: u8,
}

impl TextDefaultStyle {
    pub fn new() -> TextDefaultStyle {
        TextDefaultStyle {
            rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
            font_name: "OpenSans-Regular.ttf",
            font_size: 20u8
        }
    }
}

impl Style<Text> for TextDefaultStyle {
    fn setup_dirty_watching(&self, data: &mut Text, control: &Rc<RefCell<Control<Text>>>) {
        data.properties.text.dirty_watching(control);
    }

    fn get_preferred_size(&self, data: &Text, drawing_context: &mut DrawingContext, _size: Size) -> Size {
        let (text_width, text_height) = drawing_context.get_font_dimensions(self.font_name, self.font_size, &data.properties.text.get());
        Size::new(text_width as f32, text_height as f32)
    }

    fn set_rect(&mut self, _data: &Text, rect: Rect) {    
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Text, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) { HitTestResult::Current } else { HitTestResult::Nothing }
    }

    fn to_primitives(&self, data: &Text,
        drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let (text_width, text_height) = drawing_context.get_font_dimensions(self.font_name, self.font_size, &data.properties.text.get());

        vec.push(Primitive::Text {
            resource_key: self.font_name.to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            position: UserPixelPoint::new(x + (width - text_width as f32) / 2.0, y + (height - text_height as f32) / 2.0),
            size: self.font_size as u16,
            text: data.properties.text.get(),
        });

        vec
    }
}
