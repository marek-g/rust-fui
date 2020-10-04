#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Thickness {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Thickness {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Thickness {
            left,
            top,
            right,
            bottom,
        }
    }
}
