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
    let dpr = 1.0f32; // device pixel ratio
    let physical_pixel = 1.0 / dpr;
    let half_pixel = physical_pixel / 2.0;
    let line_thickness = 1.0f32 * physical_pixel;

    // align x & y to physical grid
    let x = (x * dpr).round() / dpr;
    let y = (y * dpr).round() / dpr;

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
        start: (x + half_pixel, y + half_pixel).into(),
        end: (x + grad_width, y + grad_height).into(),
        colors: vec![border_color1.into(), border_color2.into()],
        stops: vec![0.0, 1.0],
        tile_mode: TileMode::Clamp,
        transformation: None,
    })
    .with_draw_style(DrawStyle::Stroke)
    .with_stroke_width(line_thickness);

    let mut path_builder = DrawingPathBuilder::default();
    path_builder.move_to((x + width - half_pixel, y + half_pixel));
    path_builder.line_to((x + half_pixel, y + half_pixel));
    path_builder.line_to((x + half_pixel, y + height - half_pixel));
    let path = path_builder.build();

    display.draw_path(&path, paint);

    // border medium
    let paint = DrawingPaint::color_source(ColorSource::LinearGradient {
        start: (x + width - grad_width, y + height - grad_height).into(),
        end: (x + width - half_pixel, y + height - half_pixel).into(),
        colors: vec![border_color3.into(), border_color4.into()],
        stops: vec![0.0, 1.0],
        tile_mode: TileMode::Clamp,
        transformation: None,
    })
    .with_draw_style(DrawStyle::Stroke)
    .with_stroke_width(line_thickness);

    let mut path_builder = DrawingPathBuilder::default();
    path_builder.move_to((x + physical_pixel, y + height - half_pixel));
    path_builder.line_to((x + width - half_pixel, y + height - half_pixel));
    path_builder.line_to((x + width - half_pixel, y + physical_pixel));
    let path = path_builder.build();

    display.draw_path(&path, paint);

    // white shiny pixel
    let rect = if !is_pressed {
        rect(x, y, line_thickness, line_thickness)
    } else {
        rect(
            x + width - physical_pixel,
            y + height - physical_pixel,
            physical_pixel,
            physical_pixel,
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
    let dpr = 1.0f32; // device pixel ratio
    let physical_pixel = 1.0 / dpr;
    let half_pixel = physical_pixel / 2.0;
    let line_thickness = 1.0f32 * physical_pixel;

    // align x & y to physical grid
    let x = (x * dpr).round() / dpr;
    let y = (y * dpr).round() / dpr;

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
        start: (x + half_pixel, y + half_pixel).into(),
        end: (x + width - half_pixel, y + height - half_pixel).into(),
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
        rect(
            x + half_pixel,
            y + half_pixel,
            width - physical_pixel,
            height - physical_pixel,
        ),
        RoundingRadii::single_radii(radius),
        paint,
    );

    // white shiny pixel
    let offset = radius * 0.3; // aproximate position on arc
    let shiny_rect = if !is_pressed {
        rect(x + offset, y + offset, physical_pixel, physical_pixel)
    } else {
        rect(
            x + width - offset - physical_pixel,
            y + height - offset - physical_pixel,
            physical_pixel,
            physical_pixel,
        )
    };
    display.draw_rect(
        shiny_rect,
        [1.0, 1.0, 1.0, if !is_pressed { 1.0 } else { 0.5 }],
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
    let dpr = 1.0f32; // device pixel ratio
    let physical_pixel = 1.0 / dpr;
    let half_pixel = physical_pixel / 2.0;
    let line_thickness = 1.0f32 * physical_pixel;

    border_3d_single(
        display,
        x + physical_pixel,
        y + physical_pixel,
        width - physical_pixel * 2.0f32,
        height - physical_pixel * 2.0f32,
        is_pressed,
        is_hover,
        is_focused,
    );

    // border dark
    display.draw_rect(
        rect(
            x + half_pixel,
            y + half_pixel,
            width - physical_pixel,
            height - physical_pixel,
        ),
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
    let dpr = 1.0f32; // device pixel ratio
    let physical_pixel = 1.0 / dpr;
    let half_pixel = physical_pixel / 2.0;

    border_3d_single_rounded(
        display,
        x + physical_pixel,
        y + physical_pixel,
        width - physical_pixel * 2.0,
        height - physical_pixel * 2.0,
        radius,
        is_pressed,
        is_hover,
        is_focused,
    );

    // border dark
    display.draw_rounded_rect(
        rect(
            x + half_pixel,
            y + half_pixel,
            width - physical_pixel,
            height - physical_pixel,
        ),
        RoundingRadii::single_radii(radius + physical_pixel),
        DrawingPaint::stroke_color(BORDER_DARK, physical_pixel),
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
    mut display: &mut DrawingDisplayListBuilder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_hover: bool,
    is_focused: bool,
    fill_color: Color,
) {
    let dpr = 1.0f32; // device pixel ratio
    let physical_pixel = 1.0 / dpr;
    let half_pixel = physical_pixel / 2.0;

    border_3d_single(display, x, y, width, height, false, is_hover, is_focused);

    border_3d_single(
        display,
        x + physical_pixel * 2.0,
        y + physical_pixel * 2.0,
        width - physical_pixel * 4.0,
        height - physical_pixel * 4.0,
        true,
        is_hover,
        is_focused,
    );

    // inside border (stroke)
    display.draw_rect(
        rect(
            x + physical_pixel + half_pixel,
            y + physical_pixel + half_pixel,
            width - physical_pixel * 2.0 - physical_pixel,
            height - physical_pixel * 2.0 - physical_pixel,
        ),
        DrawingPaint::stroke_color(fill_color, physical_pixel),
    );

    shadow_under_rect(&mut display, x, y, width, height, 6.0);
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
    let dpr = 1.0f32; // device pixel ratio

    // align x & y to physical grid
    let x = (x * dpr).round() / dpr;
    let y = (y * dpr).round() / dpr;

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
    let dpr = 1.0f32; // device pixel ratio

    // align x & y to physical grid
    let x = (x * dpr).round() / dpr;
    let y = (y * dpr).round() / dpr;

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
    // smaller shadow - close to the border
    let mut path_builder = DrawingPathBuilder::default();
    path_builder.move_to((x + width, y + shadow_size + shadow_size / 2.0));
    path_builder.line_to((
        x + width + shadow_size / 2.0,
        y + shadow_size + shadow_size / 2.0,
    ));
    path_builder.line_to((
        x + width + shadow_size / 2.0,
        y + height + shadow_size / 2.0,
    ));
    path_builder.line_to((
        x + shadow_size + shadow_size / 2.0,
        y + height + shadow_size / 2.0,
    ));
    path_builder.line_to((x + shadow_size + shadow_size / 2.0, y + height));
    path_builder.line_to((x + width, y + height));
    let path = path_builder.build();

    display.draw_path(
        &path,
        DrawingPaint::color([0.0, 0.0, 0.0, 0.4]).with_mask_filter(MaskFilter::Blur {
            style: BlurStyle::Normal,
            sigma: shadow_size / 3.0,
        }),
    );

    // larger shadow
    let mut path_builder = DrawingPathBuilder::default();
    path_builder.move_to((x + width, y + shadow_size));
    path_builder.line_to((x + width + shadow_size, y + shadow_size));
    path_builder.line_to((x + width + shadow_size, y + height + shadow_size));
    path_builder.line_to((x + shadow_size, y + height + shadow_size));
    path_builder.line_to((x + shadow_size, y + height));
    path_builder.line_to((x + width, y + height));
    let path = path_builder.build();

    display.draw_path(
        &path,
        DrawingPaint::color([0.0, 0.0, 0.0, 0.2]).with_mask_filter(MaskFilter::Blur {
            style: BlurStyle::Normal,
            sigma: shadow_size,
        }),
    );
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
    // smaller shadow - approximation
    let mut path_builder = DrawingPathBuilder::default();
    path_builder.move_to((x + width, y + shadow_size + shadow_size / 2.0));
    path_builder.line_to((
        x + width + shadow_size / 2.0,
        y + shadow_size + shadow_size / 2.0 + radius / 2.0,
    ));
    path_builder.line_to((
        x + width + shadow_size / 2.0,
        y + height + shadow_size / 2.0 - radius / 2.0,
    ));
    path_builder.line_to((x + width - radius / 2.0, y + height + shadow_size / 2.0));
    path_builder.line_to((
        x + shadow_size + shadow_size / 2.0 + radius / 2.0,
        y + height + shadow_size / 2.0,
    ));
    path_builder.line_to((x + shadow_size + shadow_size / 2.0, y + height));
    path_builder.line_to((x + width - radius, y + height));
    path_builder.line_to((x + width, y + height - radius));
    let path = path_builder.build();

    display.draw_path(
        &path,
        DrawingPaint::color([0.0, 0.0, 0.0, 0.4]).with_mask_filter(MaskFilter::Blur {
            style: BlurStyle::Normal,
            sigma: shadow_size / 3.0,
        }),
    );

    // larger shadow - approximation
    let mut path_builder = DrawingPathBuilder::default();
    path_builder.move_to((x + width, y + shadow_size));
    path_builder.line_to((x + width + shadow_size, y + shadow_size + radius));
    path_builder.line_to((x + width + shadow_size, y + height - radius + shadow_size));
    path_builder.line_to((x + width - radius + shadow_size, y + height + shadow_size));
    path_builder.line_to((x + shadow_size + radius, y + height + shadow_size));
    path_builder.line_to((x + shadow_size, y + height));
    path_builder.line_to((x + width - radius, y + height));
    path_builder.line_to((x + width, y + height - radius));
    let path = path_builder.build();

    display.draw_path(
        &path,
        DrawingPaint::color([0.0, 0.0, 0.0, 0.2]).with_mask_filter(MaskFilter::Blur {
            style: BlurStyle::Normal,
            sigma: shadow_size,
        }),
    );
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
    let dpr = 1.0f32; // device pixel ratio
    let physical_pixel = 1.0 / dpr;

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
        display,
        x + physical_pixel * 2.0,
        y + physical_pixel * 2.0,
        width - physical_pixel * 4.0,
        height - physical_pixel * 4.0,
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
    let dpr = 1.0f32; // device pixel ratio
    let physical_pixel = 1.0 / dpr; // physical pixel
    let _half_pixel = physical_pixel / 2.0; // half pixel

    // align x & y to physical grid
    let x = (x * dpr).round() / dpr;
    let y = (y * dpr).round() / dpr;

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
        if is_pressed {
            3.0f32 * physical_pixel
        } else {
            6.0f32 * physical_pixel
        },
    );

    // fill is 2 physical pixels smaller (because of border_32)
    // radius is smaller by the size of the border, to keep proportions
    gradient_rect_rounded(
        display,
        x + 2.0 * physical_pixel,
        y + 2.0 * physical_pixel,
        width - 4.0 * physical_pixel,
        height - 4.0 * physical_pixel,
        (radius - 2.0 * physical_pixel).max(0.0),
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
