use crate::{Rect, Size, TypeMapKey};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Alignment {
    pub fn apply(
        size: Size,
        dest_rect: Rect,
        horizontal: Option<&Alignment>,
        vertical: Option<&Alignment>,
        default_horizontal_alignment: Alignment,
        default_vertical_alignment: Alignment,
    ) -> Rect {
        let horizontal = if let Some(h) = horizontal {
            *h
        } else {
            default_horizontal_alignment
        };

        let vertical = if let Some(v) = vertical {
            *v
        } else {
            default_vertical_alignment
        };

        let (x, width) = match horizontal {
            Alignment::Start => (dest_rect.x, size.width),
            Alignment::Center => (
                dest_rect.x + (dest_rect.width - size.width) / 2.0f32,
                size.width,
            ),
            Alignment::End => (dest_rect.x + dest_rect.width - size.width, size.width),
            Alignment::Stretch => (dest_rect.x, dest_rect.width.max(size.width)),
        };

        let (y, height) = match vertical {
            Alignment::Start => (dest_rect.y, size.height),
            Alignment::Center => (
                dest_rect.y + (dest_rect.height - size.height) / 2.0f32,
                size.height,
            ),
            Alignment::End => (dest_rect.y + dest_rect.height - size.height, size.height),
            Alignment::Stretch => (dest_rect.y, dest_rect.height.max(size.height)),
        };

        Rect::new(x, y, width, height)
    }
}

//
// Attached values
//

pub struct HorizontalAlignment;
impl TypeMapKey for HorizontalAlignment {
    type Value = Alignment;
}

pub struct VerticalAlignment;
impl TypeMapKey for VerticalAlignment {
    type Value = Alignment;
}
