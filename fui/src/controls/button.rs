use common::size::Size;
use drawing_context::DrawingContext;
use controls::control::*;
use drawing::primitive::Primitive;
use drawing::units::*;
use event::*;

pub struct ButtonProperties {
    pub text: String,
}

pub struct ButtonEvents {
    pub clicked: Event
}

pub struct Button<S: Style<ButtonProperties>> {
    pub properties: ButtonProperties,
    pub events: ButtonEvents,
    style: S,

    x: u16, y: u16, width: u16, height: u16
}

impl<S: Style<ButtonProperties>> Control for Button<S> {
    fn update_size(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.x = x; self.y = y; self.width = width; self.height = height;
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        println!("event: {:?}", event);
        true
    }

    fn get_preferred_size(&self, size: Size, drawing_context: &mut DrawingContext) -> Size {
        self.style.get_preferred_size(&self.properties, size, drawing_context)
    }
    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.style.to_primitives(&self.properties,
            self.x, self.y, self.width, self.height,
            drawing_context)
    }
}


//
// Button Default Style
//

pub struct ButtonDefaultStyle {
    font_name: &'static str,
    font_size: u8,
}

impl Style<ButtonProperties> for ButtonDefaultStyle {

    fn get_preferred_size(&self, properties: &ButtonProperties, size: Size, drawing_context: &mut DrawingContext) -> Size {
        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &properties.text);
        Size::new((text_width as f32) * 1.2, (text_height as f32) * 1.2)
    }

    fn to_primitives<'a>(&self, properties: &'a ButtonProperties,
        x: u16, y: u16, width: u16, height: u16,
        drawing_context: &mut DrawingContext) -> Vec<Primitive<'a>> {
        let mut vec = Vec::new();

        let x = x as f32;
        let y = x as f32;
        let width = width as f32;
        let height = height as f32;

        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &properties.text);

        vec.push(Primitive::Rectangle {
            color: [0.1, 1.0, 0.0, 0.2],
            rect: UserPixelRect::new(UserPixelPoint::new(x + 1.0, y + 1.0),
                UserPixelSize::new(width - 2.0, height - 2.0))
        });

        vec.push(Primitive::Line {
            color: [1.0, 1.0, 1.0, 1.0],
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: [1.0, 1.0, 1.0, 1.0],
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: [0.0, 0.0, 0.0, 1.0],
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
        });
        vec.push(Primitive::Line {
            color: [0.0, 0.0, 0.0, 1.0],
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
        });

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
//
//

impl Button<ButtonDefaultStyle> {
    pub fn new() -> Self {
        Button {
            properties: ButtonProperties { text: "Hello World!".to_string() },
            events: ButtonEvents { clicked: Event::new(||{}) },
            style: ButtonDefaultStyle { font_name: "OpenSans-Regular.ttf", font_size: 20u8 },
            x: 0, y: 0, width: 0, height: 0
        }
    }
}
