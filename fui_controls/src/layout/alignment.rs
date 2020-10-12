use fui_core::Rect;
use typemap::TypeMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Alignment {
    pub fn apply(
        rect: &mut Rect,
        dest_rect: Rect,
        map: &TypeMap,
        default_horizontal_alignment: Alignment,
        default_vertical_alignment: Alignment,
    ) {
        let horizontal = if let Some(h) = map.get::<HorizontalAlignment>() {
            *h
        } else {
            default_horizontal_alignment
        };

        let vertical = if let Some(v) = map.get::<VerticalAlignment>() {
            *v
        } else {
            default_vertical_alignment
        };

        match horizontal {
            Alignment::Start => rect.x = dest_rect.x,
            Alignment::Center => rect.x = dest_rect.x + (dest_rect.width - rect.width) / 2.0f32,
            Alignment::End => rect.x = dest_rect.x + dest_rect.width - rect.width,
            Alignment::Stretch => {
                rect.x = dest_rect.x;
                rect.width = dest_rect.width.max(rect.width);
            }
        }

        match vertical {
            Alignment::Start => rect.y = dest_rect.y,
            Alignment::Center => rect.y = dest_rect.y + (dest_rect.height - rect.height) / 2.0f32,
            Alignment::End => rect.y = dest_rect.y + dest_rect.height - rect.height,
            Alignment::Stretch => {
                rect.y = dest_rect.y;
                rect.height = dest_rect.height.max(rect.height);
            }
        }
    }
}

//
// Attached values
//

pub struct HorizontalAlignment;
impl typemap::Key for HorizontalAlignment {
    type Value = Alignment;
}

pub struct VerticalAlignment;
impl typemap::Key for VerticalAlignment {
    type Value = Alignment;
}
