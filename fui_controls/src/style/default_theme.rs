use drawing::primitive::*;
use drawing::primitive_extensions::*;
use drawing::units::*;
use fui::*;

const BORDER_LIGHT: Color = [0.45, 0.45, 0.45, 1.0];
const BORDER_MEDIUM: Color = [0.20, 0.20, 0.20, 1.0];
const BORDER_DARK: Color = [0.11, 0.11, 0.11, 1.0];

const GRADIENT_TOP_NORMAL: Color = [0.29, 0.29, 0.29, 1.0];
const GRADIENT_BOT_NORMAL: Color = [0.22, 0.22, 0.22, 1.0];
const GRADIENT_TOP_PRESSED: Color = [0.15, 0.15, 0.15, 1.0];
const GRADIENT_BOT_PRESSED: Color = [0.20, 0.20, 0.20, 1.0];
const GRADIENT_TOP_HOVER: Color = [0.40, 0.40, 0.40, 1.0];
const GRADIENT_BOT_HOVER: Color = [0.35, 0.35, 0.35, 1.0];

pub fn border_3d(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_pressed: bool,
) {
    // border light
    vec.push(Primitive::Stroke {
        path: vec![
            PathElement::MoveTo(PixelPoint::new(
                x + 1.0 + 0.5,
                y + 1.0 + 0.5 + height - 3.0 - 0.5,
            )),
            PathElement::LineTo(PixelPoint::new(x + 1.0 + 0.5, y + 1.0 + 0.5)),
            PathElement::LineTo(PixelPoint::new(
                x + 1.0 + 0.5 + width - 2.0 - 0.5,
                y + 1.0 + 0.5,
            )),
        ],
        thickness: PixelThickness::new(1.0f32),
        brush: Brush::Color {
            color: if !is_pressed {
                BORDER_LIGHT
            } else {
                BORDER_MEDIUM
            },
        },
    });

    // border medium
    vec.push(Primitive::Stroke {
        path: vec![
            PathElement::MoveTo(PixelPoint::new(
                x + 1.0 + 0.5,
                y + 1.0 + 0.5 + height - 2.0 - 0.5,
            )),
            PathElement::LineTo(PixelPoint::new(
                x + 1.0 + 0.5 + width - 2.0 - 0.5,
                y + 1.0 + 0.5 + height - 2.0 - 0.5,
            )),
            PathElement::LineTo(PixelPoint::new(
                x + 1.0 + 0.5 + width - 2.0 - 0.5,
                y + 2.0 + 0.5,
            )),
        ],
        thickness: PixelThickness::new(1.0f32),
        brush: Brush::Color {
            color: if !is_pressed {
                BORDER_MEDIUM
            } else {
                BORDER_LIGHT
            },
        },
    });

    // border dark
    vec.push(Primitive::Stroke {
        path: rect_path(PixelRect::new(
            PixelPoint::new(x + 0.5, y + 0.5),
            PixelSize::new(width - 0.5, height - 0.5),
        )),
        thickness: PixelThickness::new(1.0f32),
        brush: Brush::Color { color: BORDER_DARK },
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
    // background gradient
    vec.push(Primitive::Fill {
        path: rect_path(PixelRect::new(
            PixelPoint::new(x + 2.0 + 0.5, y + 2.0 + 0.5),
            PixelSize::new(width - 4.0 - 0.5, height - 4.0 - 0.5),
        )),
        brush: Brush::LinearGradient {
            start_point: PixelPoint::new(x + 2.0 + 0.5, y + 2.0 + 0.5),
            end_point: PixelPoint::new(x + width - 4.0 - 0.5, y + height - 4.0 - 0.5),
            inner_color: if is_pressed {
                GRADIENT_TOP_PRESSED
            } else {
                if is_hover {
                    GRADIENT_TOP_HOVER
                } else {
                    GRADIENT_TOP_NORMAL
                }
            },
            outer_color: if is_pressed {
                GRADIENT_BOT_PRESSED
            } else {
                if is_hover {
                    GRADIENT_BOT_HOVER
                } else {
                    GRADIENT_BOT_NORMAL
                }
            },
        },
    });

    border_3d(&mut vec, x, y, width, height, is_pressed);
}
