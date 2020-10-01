#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn empty() -> Self {
        Rect {
            x: 0.0f32,
            y: 0.0f32,
            width: 0.0f32,
            height: 0.0f32,
        }
    }

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}
