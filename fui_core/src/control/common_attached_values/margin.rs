use crate::{Rect, Size, Thickness, TypeMapKey};

//
// Attached values
//

pub struct Margin;
impl TypeMapKey for Margin {
    type Value = Thickness;
}

impl Margin {
    pub fn add_to_rect(rect: Rect, margin: Option<&Thickness>) -> Rect {
        let thickness = if let Some(t) = margin {
            *t
        } else {
            return rect;
        };

        let mut rect = rect;
        rect.x -= thickness.left;
        rect.y -= thickness.top;
        if rect.width.is_finite() {
            rect.width = 0.0f32.max(rect.width + thickness.left + thickness.right);
        }
        if rect.height.is_finite() {
            rect.height = 0.0f32.max(rect.height + thickness.top + thickness.bottom);
        }

        rect
    }

    pub fn add_to_size(size: Size, margin: Option<&Thickness>) -> Size {
        if let Some(t) = margin {
            Self::add_thickness_to_size(size, *t)
        } else {
            size
        }
    }

    pub fn add_thickness_to_size(mut size: Size, thickness: Thickness) -> Size {
        if size.width.is_finite() {
            size.width = 0.0f32.max(size.width + thickness.left + thickness.right);
        }
        if size.height.is_finite() {
            size.height = 0.0f32.max(size.height + thickness.top + thickness.bottom);
        }

        size
    }

    pub fn remove_from_size(size: Size, margin: Option<&Thickness>) -> Size {
        if let Some(t) = margin {
            Self::remove_thickness_from_size(size, *t)
        } else {
            size
        }
    }

    pub fn remove_thickness_from_size(mut size: Size, thickness: Thickness) -> Size {
        if size.width.is_finite() {
            size.width = 0.0f32.max(size.width - thickness.left - thickness.right);
        }
        if size.height.is_finite() {
            size.height = 0.0f32.max(size.height - thickness.top - thickness.bottom);
        }

        size
    }

    pub fn remove_from_rect(rect: Rect, margin: Option<&Thickness>) -> Rect {
        let thickness = if let Some(t) = margin {
            *t
        } else {
            return rect;
        };

        let mut rect = rect;
        rect.x += thickness.left;
        rect.y += thickness.top;
        if rect.width.is_finite() {
            rect.width = 0.0f32.max(rect.width - thickness.left - thickness.right);
        }
        if rect.height.is_finite() {
            rect.height = 0.0f32.max(rect.height - thickness.top - thickness.bottom);
        }

        rect
    }
}
