use common::size::Size;
use drawing_context::DrawingContext;
use controls::control::*;
use drawing::primitive::Primitive;
use drawing::units::*;

pub struct Button<S: Style<Button<S>>> {
    style: S,

    text: &'static str,
}

impl Button<ButtonDefaultStyle> {
    pub fn new() -> Self {
        Button {
            style: ButtonDefaultStyle { font_name: "OpenSans-Regular.ttf", font_size: 20u8 },
            text: "Hello World!"
        }
    }
}

impl<S: Style<Button<S>>> Control for Button<S> {
    fn get_style(&self) -> &Style<Self> {
        &self.style
    }
}

pub struct ButtonDefaultStyle {
    font_name: &'static str,
    font_size: u8,
}

impl Style<Button<Self>> for ButtonDefaultStyle {

    fn get_preferred_size(&self, control: &Button<ButtonDefaultStyle>, size: Size, drawing_context: &mut DrawingContext) -> Size {
        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &control.text);
        Size::new((text_width as f32) * 1.2, (text_height as f32) * 1.2)
    }

    fn to_primitives(&self, control: &Button<ButtonDefaultStyle>, size: Size, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = 200.0;
        let y = 100.0;
        let width = size.width;
        let height = size.height;

        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &control.text);

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
            text: control.text,
        });

        vec
    }

}
