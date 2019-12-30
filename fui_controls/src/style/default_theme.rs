use drawing::primitive::*;
use drawing::primitive_extensions::rounded_rect_path;
use drawing::units::*;
use fui::*;

const BACKGROUND: Color = [0.1, 1.0, 0.0, 0.2];
const BACKGROUND_PRESSED: Color = [0.1, 0.5, 0.0, 0.2];
const BACKGROUND_HOVER: Color = [0.1, 1.0, 0.0, 0.4];

const COLOR_LIGHT: Color = [1.0, 1.0, 1.0, 1.0];
const COLOR_DARK: Color = [0.0, 0.0, 0.0, 1.0];

const GRADIENT_TOP_UNFOCUSED: Color = [0.29, 0.29, 0.29, 1.0];
const GRADIENT_BOT_UNFOCUSED: Color = [0.22, 0.22, 0.22, 1.0];
const GRADIENT_TOP_PRESSED: Color = [0.16, 0.16, 0.16, 1.0];
const GRADIENT_BOT_PRESSED: Color = [0.11, 0.11, 0.11, 1.0];

const BUTTON_CORNER_RADIUS: f32 = 20.0f32;

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
        thickness: PixelThickness::new(1.0f32),
        start_point: PixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
        end_point: PixelPoint::new(x + 0.5, y + 0.5),
    });
    vec.push(Primitive::Line {
        color: line_color1,
        thickness: PixelThickness::new(1.0f32),
        start_point: PixelPoint::new(x + 0.5, y + 0.5),
        end_point: PixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
    });
    vec.push(Primitive::Line {
        color: line_color2,
        thickness: PixelThickness::new(1.0f32),
        start_point: PixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
        end_point: PixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
    });
    vec.push(Primitive::Line {
        color: line_color2,
        thickness: PixelThickness::new(1.0f32),
        start_point: PixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
        end_point: PixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
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
    // background
    vec.push(Primitive::Fill {
        path: rounded_rect_path(
            PixelRect::new(
                PixelPoint::new(x + 1.0f32, y + 1.0f32),
                PixelSize::new(width - 2.0, height - 2.0),
            ),
            BUTTON_CORNER_RADIUS - 1.0f32,
        ),
        brush: Brush::LinearGradient {
            start_point: PixelPoint::new(x, y),
            end_point: PixelPoint::new(x, y + width),
            inner_color: if is_pressed {
                GRADIENT_TOP_PRESSED
            } else {
                GRADIENT_TOP_UNFOCUSED
            },
            outer_color: if is_pressed {
                GRADIENT_BOT_PRESSED
            } else {
                GRADIENT_BOT_UNFOCUSED
            },
        },
    });

    /*
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
        rect: PixelRect::new(
            PixelPoint::new(x + 1.0, y + 1.0),
            PixelSize::new(width - 2.0, height - 2.0),
        ),
    });*/

    border_3d(&mut vec, x, y, width, height, is_pressed);
}
