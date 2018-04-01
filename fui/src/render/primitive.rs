use common::color::Color;

pub enum PrimitiveKind {
    Line {
        color: Color,
        thickness: f32,
        x1: f32, y1: f32,
        x2: f32, y2: f32
    },
    Rectangle {
        color: Color,
        x: f32, y: f32,
        width: f32, height: f32
    },
    Text {
        color: Color,
        x: f32, y: f32,
        size: f32,
        text: &'static str,
    }
}

pub struct Primitive {
    pub kind : PrimitiveKind,
}
