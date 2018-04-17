use common::size::Size;
use drawing_context::DrawingContext;
use controls::control::Control;
use drawing::primitive::Primitive;
use drawing::units::*;

pub struct Button {
    text: &'static str,
    font_name: &'static str,
    font_size: u8,

    text_size: Size,
    size: Size
}

impl Button {
    pub fn new() -> Self {
        Button { text: "Hello World!", font_name: "OpenSans-Regular.ttf", font_size: 20u8,
            text_size: Size::new(0.0, 0.0), size: Size::new(0.0, 0.0) }
    }
}

impl Control for Button {

    fn get_preferred_size(&mut self, size: Size, drawing_context: &mut DrawingContext) -> Size {
        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &self.text);
        self.text_size = Size::new(text_width as f32, text_height as f32);
        Size::new(self.text_size.width * 1.2, self.text_size.height as f32 * 1.2)
    }

    fn set_size(&mut self, size: Size, drawing_context: &mut DrawingContext) -> Size {
        self.size = size;
        size
    }

    fn to_primitives(&self) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = 200.0;
        let y = 100.0;
        let width = self.size.width;
        let height = self.size.height;

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
            position: UserPixelPoint::new(x + (width - self.text_size.width) / 2.0, y + (height - self.text_size.height) / 2.0),
            size: self.font_size as u16,
            text: self.text,
        });

        vec
    }

}   