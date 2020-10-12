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

    pub fn all(all: f32) -> Self {
        Thickness {
            left: all,
            top: all,
            right: all,
            bottom: all,
        }
    }

    pub fn sides(left_right: f32, top_bottom: f32) -> Self {
        Thickness {
            left: left_right,
            top: top_bottom,
            right: left_right,
            bottom: top_bottom,
        }
    }

    pub fn left(left: f32) -> Self {
        Thickness {
            left,
            top: 0.0f32,
            right: 0.0f32,
            bottom: 0.0f32,
        }
    }

    pub fn top(top: f32) -> Self {
        Thickness {
            left: 0.0f32,
            top,
            right: 0.0f32,
            bottom: 0.0f32,
        }
    }

    pub fn right(right: f32) -> Self {
        Thickness {
            left: 0.0f32,
            top: 0.0f32,
            right,
            bottom: 0.0f32,
        }
    }

    pub fn bottom(bottom: f32) -> Self {
        Thickness {
            left: 0.0f32,
            top: 0.0f32,
            right: 0.0f32,
            bottom,
        }
    }
}
