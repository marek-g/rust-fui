use common::*;
use drawing::primitive::Primitive;
use drawing::units::*;

const BACKGROUND: Color = [0.1, 1.0, 0.0, 0.2];
const BACKGROUND_PRESSED: Color = [0.1, 0.5, 0.0, 0.2];
const BACKGROUND_HOVER: Color = [0.1, 1.0, 0.0, 0.4];

const COLOR_LIGHT: Color = [1.0, 1.0, 1.0, 1.0];
const COLOR_DARK: Color = [0.0, 0.0, 0.0, 1.0];

pub fn border_3d(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_pressed: bool,
) {
    let line_color1 = if !is_pressed { COLOR_LIGHT } else { COLOR_DARK };
    let line_color2 = if !is_pressed { COLOR_DARK } else { COLOR_LIGHT };

    vec.push(Primitive::Line {
        color: line_color1,
        thickness: UserPixelThickness::new(1.0f32),
        start_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
        end_point: UserPixelPoint::new(x + 0.5, y + 0.5),
    });
    vec.push(Primitive::Line {
        color: line_color1,
        thickness: UserPixelThickness::new(1.0f32),
        start_point: UserPixelPoint::new(x + 0.5, y + 0.5),
        end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
    });
    vec.push(Primitive::Line {
        color: line_color2,
        thickness: UserPixelThickness::new(1.0f32),
        start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
        end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
    });
    vec.push(Primitive::Line {
        color: line_color2,
        thickness: UserPixelThickness::new(1.0f32),
        start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
        end_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
    });
}

pub fn button(
    mut vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_pressed: bool,
    is_hover: bool,
) {
    let background = if is_pressed {
        BACKGROUND_PRESSED
    } else {
        if is_hover {
            BACKGROUND_HOVER
        } else {
            BACKGROUND
        }
    };

    vec.push(Primitive::Rectangle {
        color: background,
        rect: UserPixelRect::new(
            UserPixelPoint::new(x + 1.0, y + 1.0),
            UserPixelSize::new(width - 2.0, height - 2.0),
        ),
    });

    border_3d(&mut vec, x, y, width, height, is_pressed);
}
