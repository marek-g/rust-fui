#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Size {
            width: width,
            height: height,
        }
    }

    pub fn empty() -> Self {
        Size {
            width: 0.0f32,
            height: 0.0f32,
        }
    }
}
