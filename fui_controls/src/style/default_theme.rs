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
            PathElement::MoveTo(PixelPoint::new(x + 1.0f32, y + height - 2.0f32)),
            PathElement::LineTo(PixelPoint::new(x + 1.0f32, y + 1.0f32)),
            PathElement::LineTo(PixelPoint::new(x + width, y + 1.0f32)),
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
            PathElement::MoveTo(PixelPoint::new(x + 1.0f32, y + height - 1.0f32)),
            PathElement::LineTo(PixelPoint::new(x + width - 1.0f32, y + height - 1.0f32)),
            PathElement::LineTo(PixelPoint::new(x + width - 1.0f32, y + 1.0f32)),
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
            PixelPoint::new(x, y),
            PixelSize::new(width, height),
        )),
        thickness: PixelThickness::new(1.0f32),
        brush: Brush::Color { color: BORDER_DARK },
    });
}

pub fn gradient_rect(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color_top: Color,
    color_bottom: Color,
) {
    vec.push(Primitive::Fill {
        path: rect_path(PixelRect::new(
            PixelPoint::new(x, y),
            PixelSize::new(width, height),
        )),
        brush: Brush::LinearGradient {
            start_point: PixelPoint::new(x, y),
            end_point: PixelPoint::new(x + width, y + height),
            inner_color: color_top,
            outer_color: color_bottom,
        },
    });
}

pub fn shadow_under_rect(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    shadow_size: f32,
) {
    let mut shadow_fill_path = Vec::new();
    shadow_fill_path.append(&mut rect_path(PixelRect::new(
        PixelPoint::new(x + width + 0.5f32, y + shadow_size + 0.5f32),
        PixelSize::new(shadow_size - 0.5f32, height - shadow_size - 0.5f32),
    )));
    shadow_fill_path.append(&mut rect_path(PixelRect::new(
        PixelPoint::new(x + shadow_size + 0.5f32, y + height + 0.5f32),
        PixelSize::new(width - 0.5f32, shadow_size - 0.5f32),
    )));
    vec.push(Primitive::Fill {
        path: shadow_fill_path,
        brush: Brush::ShadowGradient {
            rect: PixelRect::new(
                PixelPoint::new(x + shadow_size + 0.5f32, y + shadow_size + 0.5f32),
                PixelSize::new(width - 2.0f32, height - 2.0f32),
            ),
            radius: shadow_size,
            feather: shadow_size,
            inner_color: [0.0, 0.0, 0.0, 0.35],
            outer_color: [0.0, 0.0, 0.0, 0.0],
        },
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
    gradient_rect(&mut vec, x + 2.5f32, y + 2.5f32, width - 4.0f32, height - 4.0f32,
        if is_pressed {
            GRADIENT_TOP_PRESSED
        } else {
            if is_hover {
                GRADIENT_TOP_HOVER
            } else {
                GRADIENT_TOP_NORMAL
            }
        },
        if is_pressed {
            GRADIENT_BOT_PRESSED
        } else {
            if is_hover {
                GRADIENT_BOT_HOVER
            } else {
                GRADIENT_BOT_NORMAL
            }
        });

    border_3d(&mut vec, x + 0.5f32, y + 0.5f32, width - 1.0f32, height - 1.0f32, is_pressed);

    shadow_under_rect(&mut vec, x + 0.5f32, y + 0.5f32, width - 1.0f32, height - 1.0f32, if is_pressed { 3.0f32 } else { 6.0f32 });
}
