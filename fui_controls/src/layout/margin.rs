use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use drawing::primitive::Primitive;
use fui_core::*;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

//
// Attached values
//

pub struct Margin;
impl typemap::Key for Margin {
    type Value = Thickness;
}

impl Margin {
    pub fn add_to_rect(mut rect: Rect, map: &TypeMap) -> Rect {
        let thickness = if let Some(t) = map.get::<Margin>() {
            *t
        } else {
            return rect;
        };

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

    pub fn remove_from_size(mut size: Size, map: &TypeMap) -> Size {
        let thickness = if let Some(t) = map.get::<Margin>() {
            *t
        } else {
            return size;
        };

        if size.width.is_finite() {
            size.width = 0.0f32.max(size.width - thickness.left - thickness.right);
        }
        if size.height.is_finite() {
            size.height = 0.0f32.max(size.height - thickness.top - thickness.bottom);
        }

        size
    }

    pub fn remove_from_rect(mut rect: Rect, map: &TypeMap) -> Rect {
        let thickness = if let Some(t) = map.get::<Margin>() {
            *t
        } else {
            return rect;
        };

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
