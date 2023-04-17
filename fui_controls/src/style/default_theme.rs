use drawing::primitive::*;
use drawing::primitive_extensions::*;
use drawing::units::*;
use fui_core::*;

const BORDER_LIGHT1: Color = [0.65, 0.65, 0.65, 1.0];
const BORDER_LIGHT2: Color = [0.35, 0.35, 0.35, 1.0];
const BORDER_MEDIUM1: Color = [0.15, 0.15, 0.15, 1.0];
const BORDER_MEDIUM2: Color = [0.12, 0.12, 0.12, 1.0];
const BORDER_DARK: Color = [0.0, 0.0, 0.0, 1.0];

const GRADIENT_TOP_NORMAL: Color = [0.35, 0.35, 0.35, 1.0];
const GRADIENT_BOT_NORMAL: Color = [0.28, 0.28, 0.28, 1.0];
const HOVER_HIGHLIGHT: [f32; 3] = [1.25f32, 1.25f32, 1.25f32];
const PRESSED_HIGHLIGHT: [f32; 3] = [0.75f32, 0.75f32, 0.75f32];
const FOCUSED_HIGHLIGHT: [f32; 3] = [2.0f32, 2.0f32, 1.0f32];

pub const WINDOW_FRAME_COLOR: Color = [0.0f32, 0.4f32, 1.0f32, 1.0f32];

fn multiply_color(color: Color, factor: [f32; 3]) -> Color {
    [
        (color[0] * factor[0]).min(1.0f32),
        (color[1] * factor[1]).min(1.0f32),
        (color[2] * factor[2]).min(1.0f32),
        color[3],
    ]
}

pub fn border_3d_single(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_pressed: bool,
    is_hover: bool,
    is_focused: bool,
) {
    let line_thickness = 1.0f32;

    let w2 = width * width;
    let h2 = height * height;
    let grad_len = (w2 + h2).sqrt();
    let grad_width = h2 / grad_len;
    let grad_height = width * height / grad_len;

    let (mut border_color1, mut border_color2, mut border_color3, mut border_color4) = if is_pressed
    {
        (
            multiply_color(BORDER_MEDIUM2, PRESSED_HIGHLIGHT),
            multiply_color(BORDER_MEDIUM1, PRESSED_HIGHLIGHT),
            multiply_color(BORDER_LIGHT2, PRESSED_HIGHLIGHT),
            multiply_color(BORDER_LIGHT1, PRESSED_HIGHLIGHT),
        )
    } else {
        if is_hover {
            (
                multiply_color(BORDER_LIGHT1, HOVER_HIGHLIGHT),
                multiply_color(BORDER_LIGHT2, HOVER_HIGHLIGHT),
                multiply_color(BORDER_MEDIUM1, HOVER_HIGHLIGHT),
                multiply_color(BORDER_MEDIUM2, HOVER_HIGHLIGHT),
            )
        } else {
            (BORDER_LIGHT1, BORDER_LIGHT2, BORDER_MEDIUM1, BORDER_MEDIUM2)
        }
    };

    if is_focused {
        border_color1 = multiply_color(border_color1, FOCUSED_HIGHLIGHT);
        border_color2 = multiply_color(border_color2, FOCUSED_HIGHLIGHT);
        border_color3 = multiply_color(border_color3, FOCUSED_HIGHLIGHT);
        border_color4 = multiply_color(border_color4, FOCUSED_HIGHLIGHT);
    }

    // border light
    vec.push(Primitive::Stroke {
        path: vec![
            PathElement::MoveTo(PixelPoint::new(x + width, y + line_thickness * 0.5f32)),
            PathElement::LineTo(PixelPoint::new(
                x + line_thickness * 0.5f32,
                y + line_thickness * 0.5f32,
            )),
            PathElement::LineTo(PixelPoint::new(x + line_thickness * 0.5f32, y + height)),
        ],
        thickness: PixelThickness::new(line_thickness),
        brush: Brush::LinearGradient {
            start_point: PixelPoint::new(x, y),
            end_point: PixelPoint::new(x + grad_width, y + grad_height),
            inner_color: border_color1,
            outer_color: border_color2,
        },
    });

    // border medium
    vec.push(Primitive::Stroke {
        path: vec![
            PathElement::MoveTo(PixelPoint::new(
                x + line_thickness,
                y + height - line_thickness * 0.5f32,
            )),
            PathElement::LineTo(PixelPoint::new(
                x + width - line_thickness * 0.5f32,
                y + height - line_thickness * 0.5f32,
            )),
            PathElement::LineTo(PixelPoint::new(
                x + width - line_thickness * 0.5f32,
                y + line_thickness,
            )),
        ],
        thickness: PixelThickness::new(line_thickness),
        brush: Brush::LinearGradient {
            start_point: PixelPoint::new(x + width - grad_width, y + height - grad_height),
            end_point: PixelPoint::new(x + width, y + height),
            inner_color: border_color3,
            outer_color: border_color4,
        },
    });

    // white shiny pixel
    vec.push(Primitive::Rectangle {
        rect: PixelRect::new(
            if !is_pressed {
                PixelPoint::new(x, y)
            } else {
                PixelPoint::new(x + width - line_thickness, y + height - line_thickness)
            },
            PixelSize::new(line_thickness, line_thickness),
        ),
        color: [
            1.0f32,
            1.0f32,
            1.0f32,
            if !is_pressed { 1.0f32 } else { 0.5f32 },
        ],
    });
}

pub fn border_3d_single_rounded(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    is_pressed: bool,
    is_hover: bool,
    is_focused: bool,
) {
    let line_thickness = 1.0f32;

    let w2 = width * width;
    let h2 = height * height;
    let grad_len = (w2 + h2).sqrt();
    let grad_width = h2 / grad_len;
    let grad_height = width * height / grad_len;

    let (mut border_color1, mut border_color2, mut border_color3, mut border_color4) = if is_pressed
    {
        (
            multiply_color(BORDER_MEDIUM2, PRESSED_HIGHLIGHT),
            multiply_color(BORDER_MEDIUM1, PRESSED_HIGHLIGHT),
            multiply_color(BORDER_LIGHT2, PRESSED_HIGHLIGHT),
            multiply_color(BORDER_LIGHT1, PRESSED_HIGHLIGHT),
        )
    } else {
        if is_hover {
            (
                multiply_color(BORDER_LIGHT1, HOVER_HIGHLIGHT),
                multiply_color(BORDER_LIGHT2, HOVER_HIGHLIGHT),
                multiply_color(BORDER_MEDIUM1, HOVER_HIGHLIGHT),
                multiply_color(BORDER_MEDIUM2, HOVER_HIGHLIGHT),
            )
        } else {
            (BORDER_LIGHT1, BORDER_LIGHT2, BORDER_MEDIUM1, BORDER_MEDIUM2)
        }
    };

    if is_focused {
        border_color1 = multiply_color(border_color1, FOCUSED_HIGHLIGHT);
        border_color2 = multiply_color(border_color2, FOCUSED_HIGHLIGHT);
        border_color3 = multiply_color(border_color3, FOCUSED_HIGHLIGHT);
        border_color4 = multiply_color(border_color4, FOCUSED_HIGHLIGHT);
    }

    // border light
    vec.push(Primitive::Stroke {
        path: pixel_rect_path_rounded_half(
            PixelRect::new(PixelPoint::new(x, y), PixelSize::new(width, height)),
            PixelThickness::new(line_thickness),
            radius,
            true,
        ),
        thickness: PixelThickness::new(line_thickness),
        brush: Brush::LinearGradient {
            start_point: PixelPoint::new(x, y),
            end_point: PixelPoint::new(x + grad_width, y + grad_height),
            inner_color: border_color1,
            outer_color: border_color2,
        },
    });

    // border medium
    vec.push(Primitive::Stroke {
        path: pixel_rect_path_rounded_half(
            PixelRect::new(PixelPoint::new(x, y), PixelSize::new(width, height)),
            PixelThickness::new(line_thickness),
            radius,
            false,
        ),
        thickness: PixelThickness::new(line_thickness),
        brush: Brush::LinearGradient {
            start_point: PixelPoint::new(x + width - grad_width, y + height - grad_height),
            end_point: PixelPoint::new(x + width, y + height),
            inner_color: border_color3,
            outer_color: border_color4,
        },
    });

    // white shiny pixel
    vec.push(Primitive::Rectangle {
        rect: PixelRect::new(
            if !is_pressed {
                PixelPoint::new(x + radius / 2.0f32, y + radius / 2.0f32)
            } else {
                PixelPoint::new(
                    x + width - line_thickness * radius / 2.0f32,
                    y + height - line_thickness * radius / 2.0f32,
                )
            },
            PixelSize::new(line_thickness, line_thickness),
        ),
        color: [
            1.0f32,
            1.0f32,
            1.0f32,
            if !is_pressed { 1.0f32 } else { 0.5f32 },
        ],
    });
}

pub fn border_3d(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_pressed: bool,
    is_hover: bool,
    is_focused: bool,
) {
    let line_thickness = 1.0f32;

    border_3d_single(
        vec,
        x + line_thickness,
        y + line_thickness,
        width - line_thickness * 2.0f32,
        height - line_thickness * 2.0f32,
        is_pressed,
        is_hover,
        is_focused,
    );

    // border dark
    vec.push(Primitive::Stroke {
        path: pixel_rect_path(
            PixelRect::new(PixelPoint::new(x, y), PixelSize::new(width, height)),
            PixelThickness::new(line_thickness),
        ),
        thickness: PixelThickness::new(line_thickness),
        brush: Brush::Color { color: BORDER_DARK },
    });
}

pub fn border_3d_rounded(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    is_pressed: bool,
    is_hover: bool,
    is_focused: bool,
) {
    let line_thickness = 1.0f32;

    border_3d_single_rounded(
        vec,
        x + line_thickness,
        y + line_thickness,
        width - line_thickness * 2.0f32,
        height - line_thickness * 2.0f32,
        radius,
        is_pressed,
        is_hover,
        is_focused,
    );

    // border dark
    vec.push(Primitive::Stroke {
        path: pixel_rect_path_rounded(
            PixelRect::new(PixelPoint::new(x, y), PixelSize::new(width, height)),
            PixelThickness::new(line_thickness),
            radius + line_thickness,
        ),
        thickness: PixelThickness::new(line_thickness),
        brush: Brush::Color { color: BORDER_DARK },
    });
}

pub fn border_3d_edit(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_hover: bool,
    is_focused: bool,
) {
    let color = if is_focused {
        multiply_color([0.4f32, 0.4f32, 0.4f32, 1.0f32], FOCUSED_HIGHLIGHT)
    } else if is_hover {
        multiply_color([0.35f32, 0.35f32, 0.35f32, 1.0f32], FOCUSED_HIGHLIGHT)
    } else {
        [0.4f32, 0.4f32, 0.4f32, 1.0f32]
    };

    border_3d_with_color(vec, x, y, width, height, is_hover, is_focused, color);
}

pub fn border_3d_with_color(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_hover: bool,
    is_focused: bool,
    fill_color: Color,
) {
    border_3d_single(vec, x, y, width, height, false, is_hover, is_focused);

    border_3d_single(
        vec,
        x + 2.0f32,
        y + 2.0f32,
        width - 4.0f32,
        height - 4.0f32,
        true,
        is_hover,
        is_focused,
    );

    vec.push(Primitive::Stroke {
        path: pixel_rect_path(
            PixelRect::new(
                PixelPoint::new(x + 1.0f32, y + 1.0f32),
                PixelSize::new(width - 2.0f32, height - 2.0f32),
            ),
            PixelThickness::new(1.0f32),
        ),
        thickness: PixelThickness::new(1.0f32),
        brush: drawing::primitive::Brush::Color { color: fill_color },
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

pub fn gradient_rect_rounded(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    color_top: Color,
    color_bottom: Color,
) {
    vec.push(Primitive::Fill {
        path: rect_path_rounded(
            PixelRect::new(PixelPoint::new(x, y), PixelSize::new(width, height)),
            radius,
        ),
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
        PixelPoint::new(x + width, y),
        PixelSize::new(shadow_size, height),
    )));
    shadow_fill_path.append(&mut rect_path(PixelRect::new(
        PixelPoint::new(x, y + height),
        PixelSize::new(width + shadow_size, shadow_size),
    )));
    vec.push(Primitive::Fill {
        path: shadow_fill_path,
        brush: Brush::ShadowGradient {
            rect: PixelRect::new(
                PixelPoint::new(x + shadow_size * 0.5f32, y + shadow_size * 0.5f32),
                PixelSize::new(width, height),
            ),
            radius: shadow_size,
            feather: shadow_size * 0.5f32,
            inner_color: [0.0, 0.0, 0.0, 0.35],
            outer_color: [0.0, 0.0, 0.0, 0.0],
        },
    });
}

pub fn shadow_under_rect_rounded(
    vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    shadow_size: f32,
) {
    let mut shadow_fill_path = Vec::new();
    shadow_fill_path.append(&mut rect_path(PixelRect::new(
        PixelPoint::new(x + width, y),
        PixelSize::new(shadow_size, height - radius),
    )));
    shadow_fill_path.append(&mut rect_path(PixelRect::new(
        PixelPoint::new(x, y + height),
        PixelSize::new(width - radius, shadow_size),
    )));
    shadow_fill_path.append(&mut rect_path(PixelRect::new(
        PixelPoint::new(x + width - radius, y + height - radius),
        PixelSize::new(radius + shadow_size, radius + shadow_size),
    )));
    vec.push(Primitive::Fill {
        path: shadow_fill_path,
        brush: Brush::ShadowGradient {
            rect: PixelRect::new(
                PixelPoint::new(x + shadow_size * 0.5f32, y + shadow_size * 0.5f32),
                PixelSize::new(width, height),
            ),
            radius: shadow_size,
            feather: shadow_size * 0.5f32,
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
    _is_focused: bool,
) {
    let (gradient_top_color, gradient_bottom_color) = if is_pressed {
        (
            multiply_color(GRADIENT_BOT_NORMAL, PRESSED_HIGHLIGHT),
            multiply_color(GRADIENT_TOP_NORMAL, PRESSED_HIGHLIGHT),
        )
    } else {
        if is_hover {
            (
                multiply_color(GRADIENT_TOP_NORMAL, HOVER_HIGHLIGHT),
                multiply_color(GRADIENT_BOT_NORMAL, HOVER_HIGHLIGHT),
            )
        } else {
            (GRADIENT_TOP_NORMAL, GRADIENT_BOT_NORMAL)
        }
    };

    gradient_rect(
        &mut vec,
        x + 2.0f32,
        y + 2.0f32,
        width - 4.0f32,
        height - 4.0f32,
        gradient_top_color,
        gradient_bottom_color,
    );

    border_3d(&mut vec, x, y, width, height, is_pressed, is_hover, false);

    shadow_under_rect(
        &mut vec,
        x,
        y,
        width,
        height,
        if is_pressed { 3.0f32 } else { 6.0f32 },
    );
}

pub fn button_rounded(
    mut vec: &mut Vec<Primitive>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    is_pressed: bool,
    is_hover: bool,
    _is_focused: bool,
) {
    let (gradient_top_color, gradient_bottom_color) = if is_pressed {
        (
            multiply_color(GRADIENT_BOT_NORMAL, PRESSED_HIGHLIGHT),
            multiply_color(GRADIENT_TOP_NORMAL, PRESSED_HIGHLIGHT),
        )
    } else {
        if is_hover {
            (
                multiply_color(GRADIENT_TOP_NORMAL, HOVER_HIGHLIGHT),
                multiply_color(GRADIENT_BOT_NORMAL, HOVER_HIGHLIGHT),
            )
        } else {
            (GRADIENT_TOP_NORMAL, GRADIENT_BOT_NORMAL)
        }
    };

    shadow_under_rect_rounded(
        &mut vec,
        x,
        y,
        width,
        height,
        radius,
        if is_pressed { 3.0f32 } else { 6.0f32 },
    );

    gradient_rect_rounded(
        &mut vec,
        x + 2.0f32,
        y + 2.0f32,
        width - 4.0f32,
        height - 4.0f32,
        radius - 2.0f32,
        gradient_top_color,
        gradient_bottom_color,
    );

    border_3d_rounded(
        &mut vec, x, y, width, height, radius, is_pressed, is_hover, false,
    );
}
