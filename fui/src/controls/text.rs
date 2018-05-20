use control::*;
use common::rect::Rect;
use common::size::Size;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };

pub struct TextProperties {
    pub text: String,
}

pub struct Text {
    pub properties: TextProperties,
    style: Box<Style<TextProperties>>,

    rect: Rect,
}

impl Text {
    pub fn new(text: String) -> Self {
        Text {
            properties: TextProperties { text: text },
            style: Box::new(TextDefaultStyle { font_name: "OpenSans-Regular.ttf", font_size: 20u8 }),
            rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
        }
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

    fn set_size(&mut self, rect: Rect) {
        self.rect = rect;
        self.style.set_size(&mut self.properties, rect);
    }

    fn get_size(&self) -> Rect {
        self.rect
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        true
    }
}


//
// Text Default Style
//

pub struct TextDefaultStyle {
    font_name: &'static str,
    font_size: u8,
}

impl Style<TextProperties> for TextDefaultStyle {
    fn get_preferred_size(&self, properties: &TextProperties, drawing_context: &mut DrawingContext, _size: Size) -> Size {
        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &properties.text);
        Size::new(text_width as f32, text_height as f32)
    }

    fn set_size(&mut self, properties: &mut TextProperties, rect: Rect) {    
    }

    fn to_primitives<'a>(&self, properties: &'a TextProperties,
        drawing_context: &mut DrawingContext, rect: Rect) -> Vec<Primitive<'a>> {
        let mut vec = Vec::new();

        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &properties.text);

        vec.push(Primitive::Text {
            resource_key: self.font_name,
            color: [1.0, 1.0, 1.0, 1.0],
            position: UserPixelPoint::new(x + (width - text_width as f32) / 2.0, y + (height - text_height as f32) / 2.0),
            size: self.font_size as u16,
            text: &properties.text,
        });

        vec
    }
}


//
// object safe trait
//

impl ControlObject for Text {
    fn set_size(&mut self, rect: Rect) {
        (self as &mut Control<Properties = TextProperties>).set_size(rect)
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        (self as &mut Control<Properties = TextProperties>).handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.get_style().get_preferred_size(self.get_properties(), drawing_context, size)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_style().to_primitives(&self.get_properties(),
            drawing_context, self.get_size())
    }
}
