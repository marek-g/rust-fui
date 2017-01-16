use common::size::Size;
use backend::renderer::Renderer;
use controls::control::Control;
use render::primitive::{Primitive, PrimitiveKind};

pub struct Button {
    text: &'static str,
    font_size: f32,

    text_width: f32,
    size: Size
}

impl Button {
    pub fn new() -> Self {
        Button { text: "Hello World!", font_size: 20.0, text_width: 0.0, size: Size::new(0.0, 0.0) }
    }
}

impl Control for Button {

    fn get_preferred_size(&mut self, size: Size, renderer: &mut Renderer) -> Size {
        self.text_width = renderer.text_width(self.font_size, self.text);
        Size { width: self.text_width * 1.2, height: self.font_size * 1.2 }
    }

    fn set_size(&mut self, size: Size, renderer: &mut Renderer) -> Size {
        self.size = size;
        size
    }

    fn to_primitives(&self) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = 200.0;
        let y = 100.0;
        let width = self.size.width;
        let height = self.size.height;

        vec.push(Primitive {
            kind: PrimitiveKind::Rectangle {
                color: [0.5, 1.0, 0.0, 0.0],
                x: x, y: y,
                width: width - 1.0, height: height - 1.0
            }
        });

        vec.push(Primitive {
            kind: PrimitiveKind::Line {
                color: [1.0, 1.0, 1.0, 1.0],
                thickness: 1.0,
                x1: x, y1: y + height - 1.0,
                x2: x, y2: y,
            }
        });
        vec.push(Primitive {
            kind: PrimitiveKind::Line {
                color: [1.0, 1.0, 1.0, 1.0],
                thickness: 1.0,
                x1: x, y1: y,
                x2: x + width - 1.0, y2: y,
            }
        });
        vec.push(Primitive {
            kind: PrimitiveKind::Line {
                color: [1.0, 0.0, 0.0, 0.0],
                thickness: 1.0,
                x1: x, y1: y + height - 1.0,
                x2: x + width - 1.0, y2: y + height - 1.0,
            }
        });
        vec.push(Primitive {
            kind: PrimitiveKind::Line {
                color: [1.0, 0.0, 0.0, 0.0],
                thickness: 1.0,
                x1: x + width - 1.0, y1: y + height - 1.0,
                x2: x + width - 1.0, y2: y,
            }
        });

        vec.push(Primitive {
            kind: PrimitiveKind::Text {
                color: [1.0, 0.0, 0.0, 0.0],
                x: x + (width - self.text_width) / 2.0, y: y + (height - self.font_size) / 2.0,
                size: self.font_size,
                text: self.text,
            }
        });

        vec
    }

}