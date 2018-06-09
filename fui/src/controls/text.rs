use std::cell::RefCell;
use std::rc::Rc;

use control::*;
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
    style: Box<Style<TextProperties>>,
}

impl Text {
    pub fn new(text: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Text {
            properties: TextProperties { text: Property::new(text) },
            style: Box::new(TextDefaultStyle {
                rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
                font_name: "OpenSans-Regular.ttf",
                font_size: 20u8
            }),
        }))
    }
}

impl Control for Text {
    type Properties = TextProperties;

    fn get_properties(&self) -> &Self::Properties {
        &self.properties
    }

    fn get_style(&self) -> &Box<Style<Self::Properties>> {
        &self.style
    }

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        Vec::new()
    }

    fn is_hit_test_visible(&self) -> bool {
        false
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        true
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

impl Style<TextProperties> for TextDefaultStyle {
    fn get_preferred_size(&self, properties: &TextProperties, drawing_context: &mut DrawingContext, _size: Size) -> Size {
        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &properties.text.get());
        Size::new(text_width as f32, text_height as f32)
    }

    fn set_rect(&mut self, properties: &mut TextProperties, rect: Rect) {    
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn to_primitives(&self, properties: &TextProperties,
        drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &properties.text.get());

        vec.push(Primitive::Text {
            resource_key: self.font_name.to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            position: UserPixelPoint::new(x + (width - text_width as f32) / 2.0, y + (height - text_height as f32) / 2.0),
            size: self.font_size as u16,
            text: properties.text.get(),
        });

        vec
    }
}


//
// object safe trait
//

impl ControlObject for Text {
    fn set_rect(&mut self, rect: Rect) {
        let style = &mut self.style;
        let properties = &mut self.properties;
        style.set_rect(properties, rect);
    }

    fn get_rect(&self) -> Rect {
        self.get_style().get_rect()
    }

    fn is_hit_test_visible(&self) -> bool {
        (self as &Control<Properties = TextProperties>).is_hit_test_visible()
    }

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        (self as &mut Control<Properties = TextProperties>).get_children()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        (self as &mut Control<Properties = TextProperties>).handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.get_style().get_preferred_size(self.get_properties(), drawing_context, size)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_style().to_primitives(&self.get_properties(),
            drawing_context)
    }
}
