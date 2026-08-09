use fui_drawing::*;

// ============================================================================
// Default values for inherited attached properties
// ============================================================================

/// Default foreground color (text)
pub const DEFAULT_FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

/// Default background color (transparent)
pub const DEFAULT_BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

/// Default edit text color
pub const DEFAULT_EDIT_TEXT_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// Default font size
pub const DEFAULT_FONT_SIZE: f32 = 20.0;

/// Default font family
pub const DEFAULT_FONT_FAMILY: &str = "sans-serif";

// ============================================================================
// Border colors (used in 3D border drawing)
// ============================================================================

pub const BORDER_LIGHT1: [f32; 4] = [0.65, 0.65, 0.65, 1.0];
pub const BORDER_LIGHT2: [f32; 4] = [0.35, 0.35, 0.35, 1.0];
pub const BORDER_MEDIUM1: [f32; 4] = [0.15, 0.15, 0.15, 1.0];
pub const BORDER_MEDIUM2: [f32; 4] = [0.12, 0.12, 0.12, 1.0];
pub const BORDER_DARK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

// ============================================================================
// Gradient colors (used in button drawing)
// ============================================================================

pub const GRADIENT_TOP_NORMAL: [f32; 4] = [0.35, 0.35, 0.35, 1.0];
pub const GRADIENT_BOT_NORMAL: [f32; 4] = [0.28, 0.28, 0.28, 1.0];

// ============================================================================
// Highlight multipliers (used in 3D border/button drawing)
// ============================================================================

pub const HOVER_HIGHLIGHT: [f32; 3] = [1.25, 1.25, 1.25];
pub const PRESSED_HIGHLIGHT: [f32; 3] = [0.75, 0.75, 0.75];
pub const FOCUSED_HIGHLIGHT: [f32; 3] = [2.0, 2.0, 1.0];

// ============================================================================
// Window frame color
// ============================================================================

pub const WINDOW_FRAME_COLOR: [f32; 4] = [0.0, 0.4, 1.0, 1.0];

// ============================================================================
// Control-specific colors
// ============================================================================

/// Progress bar foreground color
pub const PROGRESS_BAR_FOREGROUND: [f32; 4] = [1.0, 0.8, 0.0, 0.75];

/// Progress bar background color
pub const PROGRESS_BAR_BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 0.25];

/// Scroll bar background color
pub const SCROLL_BAR_BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 0.25];

/// Menu text foreground
pub const MENU_FOREGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// Menu background color
pub const MENU_BACKGROUND: [f32; 4] = [1.0, 1.0, 1.0, 0.8];

/// Menu hover background color
pub const MENU_HOVER_BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 0.8];

/// TextBox border color
pub const TEXT_BOX_BORDER_COLOR: [f32; 4] = [0.4, 0.4, 0.4, 1.0];

/// Text selection background color
pub const TEXT_SELECTION_BACKGROUND: [f32; 4] = [0.0, 0.47, 0.83, 0.35];

/// Text cursor color
pub const TEXT_CURSOR_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

/// Busy indicator overlay color
pub const BUSY_INDICATOR_OVERLAY: [f32; 4] = [0.0, 0.0, 0.0, 0.7];

// ============================================================================
// Size defaults
// ============================================================================

/// Border size
pub const BORDER_SIZE: f32 = 1.0;

/// Check box button size
pub const CHECK_BOX_BUTTON_SIZE: f32 = 24.0;

/// Check box margin
pub const CHECK_BOX_MARGIN: f32 = 6.0;

/// Radio button size
pub const RADIO_BUTTON_SIZE: f32 = 24.0;

/// Radio bullet size
pub const RADIO_BULLET_SIZE: f32 = 14.0;

/// Radio margin
pub const RADIO_MARGIN: f32 = 6.0;

/// Scroll bar start margin
pub const SCROLL_BAR_START_MARGIN: f32 = 1.0;

/// Scroll bar end margin
pub const SCROLL_BAR_END_MARGIN: f32 = 1.0;

/// Scroll bar side margin
pub const SCROLL_BAR_SIDE_MARGIN: f32 = 1.0;

/// Scroll bar minimum thumb size
pub const SCROLL_BAR_MIN_THUMB_SIZE: f32 = 20.0;

/// Progress bar start margin
pub const PROGRESS_BAR_START_MARGIN: f32 = 1.0;

/// Progress bar end margin
pub const PROGRESS_BAR_END_MARGIN: f32 = 1.0;

/// Progress bar side margin
pub const PROGRESS_BAR_SIDE_MARGIN: f32 = 1.0;

/// Progress bar minimum size
pub const PROGRESS_BAR_MIN_SIZE: f32 = 22.0;

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
    let colors = vec![
        [0.0, 0.0, 0.0, 0.6].into(),
        [0.0, 0.0, 0.0, 0.2].into(),
        [0.0, 0.0, 0.0, 0.0].into(),
    ];
    let stops = vec![0.0, 0.4, 1.0];

    // right part (linear gradient)
    let bar = rect(
        x + width,
        y + shadow_size,
        shadow_size,
        height - shadow_size,
    );
    let gradient = ColorSource::LinearGradient {
        start: (x + width, y).into(),
        end: (x + width + shadow_size, y).into(),
        colors: colors.clone(),
        stops: stops.clone(),
        tile_mode: TileMode::Clamp,
        transformation: None,
    };
    display.draw_rect(bar, gradient);

    // bottom part (linear gradient)
    let bar = rect(
        x + shadow_size,
        y + height,
        width - shadow_size,
        shadow_size,
    );
    let gradient = ColorSource::LinearGradient {
        start: (x, y + height).into(),
        end: (x, y + height + shadow_size).into(),
        colors: colors.clone(),
        stops: stops.clone(),
        tile_mode: TileMode::Clamp,
        transformation: None,
    };
    display.draw_rect(bar, gradient);

    // right-bottom corner (radial gradient)
    let corner_rect = rect(x + width, y + height, shadow_size, shadow_size);
    let gradient = ColorSource::RadialGradient {
        center: (x + width, y + height).into(),
        radius: shadow_size,
        colors: colors.clone(),
        stops: stops.clone(),
        tile_mode: TileMode::Clamp,
        transformation: None,
    };
    display.draw_rect(corner_rect, gradient);

    // left-bottom corner (radial gradient)
    let corner_rect = rect(x, y + height, shadow_size, shadow_size);
    let gradient = ColorSource::RadialGradient {
        center: (x + shadow_size, y + height).into(),
        radius: shadow_size,
        colors: colors.clone(),
        stops: stops.clone(),
        tile_mode: TileMode::Clamp,
        transformation: None,
    };
    display.draw_rect(corner_rect, gradient);

    // right-top corner (radial gradient)
    let corner_rect = rect(x + width, y, shadow_size, shadow_size);
    let gradient = ColorSource::RadialGradient {
        center: (x + width, y + shadow_size).into(),
        radius: shadow_size,
        colors: colors.clone(),
        stops: stops.clone(),
        tile_mode: TileMode::Clamp,
        transformation: None,
    };
    display.draw_rect(corner_rect, gradient);
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
    let colors = vec![
        [0.0, 0.0, 0.0, 0.6].into(),
        [0.0, 0.0, 0.0, 0.2].into(),
        [0.0, 0.0, 0.0, 0.0].into(),
    ];
    let stops = vec![0.0, 0.4, 1.0];

    let full_radius = radius + shadow_size;
    let radial_stops = vec![
        radius / full_radius,
        (radius + 0.4 * shadow_size) / full_radius,
        1.0,
    ];

    // right part (linear gradient)
    display.draw_rect(
        rect(x + width, y + radius, shadow_size, height - 2.0 * radius),
        ColorSource::LinearGradient {
            start: (x + width, y).into(),
            end: (x + width + shadow_size, y).into(),
            colors: colors.clone(),
            stops: stops.clone(),
            tile_mode: TileMode::Clamp,
            transformation: None,
        },
    );

    // bottom part (linear gradient)
    display.draw_rect(
        rect(x + radius, y + height, width - 2.0 * radius, shadow_size),
        ColorSource::LinearGradient {
            start: (x, y + height).into(),
            end: (x, y + height + shadow_size).into(),
            colors: colors.clone(),
            stops: stops.clone(),
            tile_mode: TileMode::Clamp,
            transformation: None,
        },
    );

    // right-bottom corner (radial gradient)
    display.draw_rect(
        rect(
            x + width - radius,
            y + height - radius,
            full_radius,
            full_radius,
        ),
        ColorSource::RadialGradient {
            center: (x + width - radius, y + height - radius).into(),
            radius: full_radius,
            colors: colors.clone(),
            stops: radial_stops.clone(),
            tile_mode: TileMode::Clamp,
            transformation: None,
        },
    );

    // left-bottom corner (radial gradient)
    display.draw_rect(
        rect(x, y + height - radius, radius, full_radius), // szerokość ograniczona do radius
        ColorSource::RadialGradient {
            center: (x + radius, y + height - radius).into(),
            radius: full_radius,
            colors: colors.clone(),
            stops: radial_stops.clone(),
            tile_mode: TileMode::Clamp,
            transformation: None,
        },
    );

    // right-top corner (radial gradient)
    display.draw_rect(
        rect(x + width - radius, y, full_radius, radius), // wysokość ograniczona do radius
        ColorSource::RadialGradient {
            center: (x + width - radius, y + radius).into(),
            radius: full_radius,
            colors: colors.clone(),
            stops: radial_stops,
            tile_mode: TileMode::Clamp,
            transformation: None,
        },
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
