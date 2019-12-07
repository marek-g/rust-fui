use crate::common::Rect;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x: x, y: y }
    }

    pub fn is_inside(&self, rect: &Rect) -> bool {
        self.x >= rect.x
            && self.x <= rect.x + rect.width
            && self.y >= rect.y
            && self.y <= rect.y + rect.height
    }
}
