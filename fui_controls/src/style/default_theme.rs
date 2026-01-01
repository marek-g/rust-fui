use fui_drawing::*;

const BORDER_LIGHT1: [f32; 4] = [0.65, 0.65, 0.65, 1.0];
const BORDER_LIGHT2: [f32; 4] = [0.35, 0.35, 0.35, 1.0];
const BORDER_MEDIUM1: [f32; 4] = [0.15, 0.15, 0.15, 1.0];
const BORDER_MEDIUM2: [f32; 4] = [0.12, 0.12, 0.12, 1.0];
const BORDER_DARK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const GRADIENT_TOP_NORMAL: [f32; 4] = [0.35, 0.35, 0.35, 1.0];
const GRADIENT_BOT_NORMAL: [f32; 4] = [0.28, 0.28, 0.28, 1.0];
const HOVER_HIGHLIGHT: [f32; 3] = [1.25f32, 1.25f32, 1.25f32];
const PRESSED_HIGHLIGHT: [f32; 3] = [0.75f32, 0.75f32, 0.75f32];
const FOCUSED_HIGHLIGHT: [f32; 3] = [2.0f32, 2.0f32, 1.0f32];

pub const WINDOW_FRAME_COLOR: [f32; 4] = [0.0f32, 0.4f32, 1.0f32, 1.0f32];

fn multiply_color(color: [f32; 4], factor: [f32; 3]) -> [f32; 4] {
    [
        (color[0] * factor[0]).min(1.0f32),
        (color[1] * factor[1]).min(1.0f32),
        (color[2] * factor[2]).min(1.0f32),
        color[3],
    ]
}

pub fn border_3d_single(
    display: &mut DrawingDisplayListBuilder,
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
    let paint = DrawingPaint::color_source(ColorSource::LinearGradient {
        start: (x, y).into(),
        end: (x + grad_width, y + grad_height).into(),
        colors: vec![border_color1.into(), border_color2.into()],
        stops: vec![0.0, 1.0],
        tile_mode: TileMode::Clamp,
        transformation: None,
    })
    .with_draw_style(DrawStyle::Stroke)
    .with_stroke_width(line_thickness);

    let mut path_builder = DrawingPathBuilder::default();
    path_builder.move_to((x + width, y + line_thickness * 0.5f32));
    path_builder.line_to((x + line_thickness * 0.5f32, y + line_thickness * 0.5f32));
    path_builder.line_to((x + line_thickness * 0.5f32, y + height));
    let path = path_builder.build();

    display.draw_path(&path, paint);

    // border medium
    let paint = DrawingPaint::color_source(ColorSource::LinearGradient {
        start: (x + width - grad_width, y + height - grad_height).into(),
        end: (x + width, y + height).into(),
        colors: vec![border_color3.into(), border_color4.into()],
        stops: vec![0.0, 1.0],
        tile_mode: TileMode::Clamp,
        transformation: None,
    })
    .with_draw_style(DrawStyle::Stroke)
    .with_stroke_width(line_thickness);

    let mut path_builder = DrawingPathBuilder::default();
    path_builder.move_to((x + line_thickness, y + height - line_thickness * 0.5f32));
    path_builder.line_to((
        x + width - line_thickness * 0.5f32,
        y + height - line_thickness * 0.5f32,
    ));
    path_builder.line_to((x + width - line_thickness * 0.5f32, y + line_thickness));
    let path = path_builder.build();

    display.draw_path(&path, paint);

    // white shiny pixel
    let rect = if !is_pressed {
        rect(x, y, line_thickness, line_thickness)
    } else {
        rect(
            x + width - line_thickness,
            y + height - line_thickness,
            line_thickness,
            line_thickness,
        )
    };
    display.draw_rect(
        rect,
        [
            1.0f32,
            1.0f32,
            1.0f32,
            if !is_pressed { 1.0f32 } else { 0.5f32 },
        ],
    );
}

pub fn border_3d_single_rounded(
    display: &mut DrawingDisplayListBuilder,
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
    let paint = DrawingPaint::color_source(ColorSource::LinearGradient {
        start: (x, y).into(),
        end: (x + width, y + height).into(),
        colors: vec![
            border_color1.into(),
            border_color2.into(),
            border_color3.into(),
            border_color4.into(),
        ],
        stops: vec![0.0, 0.33, 0.66, 1.0],
        tile_mode: TileMode::Clamp,
        transformation: None,
    })
    .with_draw_style(DrawStyle::Stroke)
    .with_stroke_width(line_thickness);

    display.draw_rounded_rect(
        rect(x, y, width, height),
        RoundingRadii::single_radii(radius),
        paint,
    );

    // white shiny pixel
    let rect = if !is_pressed {
        rect(
            x + radius / 2.0f32,
            y + radius / 2.0f32,
            line_thickness,
            line_thickness,
        )
    } else {
        rect(
            x + width - line_thickness * radius / 2.0f32,
            y + height - line_thickness * radius / 2.0f32,
            line_thickness,
            line_thickness,
        )
    };
    display.draw_rect(
        rect,
        [
            1.0f32,
            1.0f32,
            1.0f32,
            if !is_pressed { 1.0f32 } else { 0.5f32 },
        ],
    );
}

pub fn border_3d(
    display: &mut DrawingDisplayListBuilder,
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
        display,
        x + line_thickness,
        y + line_thickness,
        width - line_thickness * 2.0f32,
        height - line_thickness * 2.0f32,
        is_pressed,
        is_hover,
        is_focused,
    );

    // border dark
    display.draw_rect(
        rect(x, y, width, height),
        DrawingPaint::stroke_color(BORDER_DARK, line_thickness),
    );
}

pub fn border_3d_rounded(
    display: &mut DrawingDisplayListBuilder,
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
        display,
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
    display.draw_rounded_rect(
        rect(x, y, width, height),
        RoundingRadii::single_radii(radius + line_thickness),
        DrawingPaint::stroke_color(BORDER_DARK, line_thickness),
    );
}

pub fn border_3d_edit(
    display: &mut DrawingDisplayListBuilder,
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

    border_3d_with_color(
        display,
        x,
        y,
        width,
        height,
        is_hover,
        is_focused,
        color.into(),
    );
}

pub fn border_3d_with_color(
    display: &mut DrawingDisplayListBuilder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_hover: bool,
    is_focused: bool,
    fill_color: Color,
) {
    border_3d_single(display, x, y, width, height, false, is_hover, is_focused);

    border_3d_single(
        display,
        x + 2.0f32,
        y + 2.0f32,
        width - 4.0f32,
        height - 4.0f32,
        true,
        is_hover,
        is_focused,
    );

    display.draw_rect(
        rect(x + 1.0f32, y + 1.0f32, width - 2.0f32, height - 2.0f32),
        DrawingPaint::stroke_color(fill_color, 1.0f32),
    );
}

pub fn gradient_rect(
    display: &mut DrawingDisplayListBuilder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color_top: Color,
    color_bottom: Color,
) {
    let paint = DrawingPaint::color_source(ColorSource::LinearGradient {
        start: (x, y).into(),
        end: (x + width, y + height).into(),
        colors: vec![color_top, color_bottom],
        stops: vec![0.0, 1.0],
        tile_mode: TileMode::Clamp,
        transformation: None,
    });

    display.draw_rect(rect(x, y, width, height), paint);
}

pub fn gradient_rect_rounded(
    display: &mut DrawingDisplayListBuilder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    color_top: Color,
    color_bottom: Color,
) {
    let paint = DrawingPaint::color_source(ColorSource::LinearGradient {
        start: (x, y).into(),
        end: (x + width, y + height).into(),
        colors: vec![color_top, color_bottom],
        stops: vec![0.0, 1.0],
        tile_mode: TileMode::Clamp,
        transformation: None,
    });

    display.draw_rounded_rect(
        rect(x, y, width, height),
        RoundingRadii::single_radii(radius),
        paint,
    );
}

pub fn shadow_under_rect(
    display: &mut DrawingDisplayListBuilder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    shadow_size: f32,
) {
    let mut path_builder = DrawingPathBuilder::default();
    //path_builder.add_rect(rect(x + width, y, shadow_size, height));
    //path_builder.add_rect(rect(x, y + height, width + shadow_size, shadow_size));
    path_builder.add_rect(rect(x, y, width, height));
    let path = path_builder.build();

    display.draw_shadow(&path, [0.0, 0.0, 0.0, 0.35], shadow_size, true, 1.0);
}

pub fn shadow_under_rect_rounded(
    display: &mut DrawingDisplayListBuilder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    shadow_size: f32,
) {
    let mut path_builder = DrawingPathBuilder::default();
    path_builder.add_rounded_rect(
        rect(x, y, width, height),
        &RoundingRadii::single_radii(radius),
    );
    let path = path_builder.build();

    display.draw_shadow(&path, [0.0, 0.0, 0.0, 0.35], shadow_size, true, 1.0);

    /*let mut shadow_fill_path = Vec::new();
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
    display.push(Primitive::Fill {
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
    });*/
}

pub fn button(
    mut display: &mut DrawingDisplayListBuilder,
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
        &mut display,
        x + 2.0f32,
        y + 2.0f32,
        width - 4.0f32,
        height - 4.0f32,
        gradient_top_color.into(),
        gradient_bottom_color.into(),
    );

    border_3d(
        &mut display,
        x,
        y,
        width,
        height,
        is_pressed,
        is_hover,
        false,
    );

    shadow_under_rect(
        &mut display,
        x,
        y,
        width,
        height,
        if is_pressed { 3.0f32 } else { 6.0f32 },
    );
}

pub fn button_rounded(
    mut display: &mut DrawingDisplayListBuilder,
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
        &mut display,
        x,
        y,
        width,
        height,
        radius,
        if is_pressed { 3.0f32 } else { 6.0f32 },
    );

    gradient_rect_rounded(
        &mut display,
        x + 2.0f32,
        y + 2.0f32,
        width - 4.0f32,
        height - 4.0f32,
        radius - 2.0f32,
        gradient_top_color.into(),
        gradient_bottom_color.into(),
    );

    border_3d_rounded(
        &mut display,
        x,
        y,
        width,
        height,
        radius,
        is_pressed,
        is_hover,
        false,
    );
}
