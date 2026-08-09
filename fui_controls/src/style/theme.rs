use fui_core::{Property, TypeMapKey};
use fui_drawing::Color;

// ============================================================================
// Inherited Attached Properties for Theming
// ============================================================================

/// Foreground color (for text and controls)
pub struct Foreground;
impl TypeMapKey for Foreground {
    type Value = Property<Color>;
}

/// Font family for text
pub struct FontFamily;
impl TypeMapKey for FontFamily {
    type Value = Property<String>;
}

/// Font size for text
pub struct FontSize;
impl TypeMapKey for FontSize {
    type Value = Property<f32>;
}

/// Background color for controls
pub struct Background;
impl TypeMapKey for Background {
    type Value = Property<Color>;
}

/// Border color
pub struct BorderColor;
impl TypeMapKey for BorderColor {
    type Value = Property<Color>;
}

// ============================================================================
// Border Colors
// ============================================================================

/// Border color variants for 3D borders
pub struct BorderLight1;
impl TypeMapKey for BorderLight1 {
    type Value = Property<Color>;
}

pub struct BorderLight2;
impl TypeMapKey for BorderLight2 {
    type Value = Property<Color>;
}

pub struct BorderMedium1;
impl TypeMapKey for BorderMedium1 {
    type Value = Property<Color>;
}

pub struct BorderMedium2;
impl TypeMapKey for BorderMedium2 {
    type Value = Property<Color>;
}

pub struct BorderDark;
impl TypeMapKey for BorderDark {
    type Value = Property<Color>;
}

// ============================================================================
// Gradient Colors
// ============================================================================

/// Gradient colors for button backgrounds
pub struct GradientTopNormal;
impl TypeMapKey for GradientTopNormal {
    type Value = Property<Color>;
}

pub struct GradientBotNormal;
impl TypeMapKey for GradientBotNormal {
    type Value = Property<Color>;
}

// ============================================================================
// Highlight Multipliers
// ============================================================================

/// Color multipliers for hover state
pub struct HoverHighlight;
impl TypeMapKey for HoverHighlight {
    type Value = Property<[f32; 3]>;
}

/// Color multipliers for pressed state
pub struct PressedHighlight;
impl TypeMapKey for PressedHighlight {
    type Value = Property<[f32; 3]>;
}

/// Color multipliers for focused state
pub struct FocusedHighlight;
impl TypeMapKey for FocusedHighlight {
    type Value = Property<[f32; 3]>;
}

// ============================================================================
// Control Specific Colors
// ============================================================================

/// Progress bar foreground color
pub struct ProgressBarForeground;
impl TypeMapKey for ProgressBarForeground {
    type Value = Property<Color>;
}

/// Progress bar background color
pub struct ProgressBarBackground;
impl TypeMapKey for ProgressBarBackground {
    type Value = Property<Color>;
}

/// Scroll bar background color
pub struct ScrollBarBackground;
impl TypeMapKey for ScrollBarBackground {
    type Value = Property<Color>;
}

/// Menu background color
pub struct MenuBackground;
impl TypeMapKey for MenuBackground {
    type Value = Property<Color>;
}

/// Menu hover background color
pub struct MenuHoverBackground;
impl TypeMapKey for MenuHoverBackground {
    type Value = Property<Color>;
}

/// TextBox edit border color
pub struct TextBoxBorderColor;
impl TypeMapKey for TextBoxBorderColor {
    type Value = Property<Color>;
}

/// Text selection background color
pub struct TextSelectionBackground;
impl TypeMapKey for TextSelectionBackground {
    type Value = Property<Color>;
}

/// Text cursor color
pub struct TextCursorColor;
impl TypeMapKey for TextCursorColor {
    type Value = Property<Color>;
}

/// Busy indicator overlay color
pub struct BusyIndicatorOverlay;
impl TypeMapKey for BusyIndicatorOverlay {
    type Value = Property<Color>;
}

// ============================================================================
// Size Constants as Attached Properties
// ============================================================================

/// Check box button size
pub struct CheckBoxButtonSize;
impl TypeMapKey for CheckBoxButtonSize {
    type Value = Property<f32>;
}

/// Check box margin
pub struct CheckBoxMargin;
impl TypeMapKey for CheckBoxMargin {
    type Value = Property<f32>;
}

/// Radio button size
pub struct RadioButtonSize;
impl TypeMapKey for RadioButtonSize {
    type Value = Property<f32>;
}

/// Radio bullet size
pub struct RadioBulletSize;
impl TypeMapKey for RadioBulletSize {
    type Value = Property<f32>;
}

/// Radio margin
pub struct RadioMargin;
impl TypeMapKey for RadioMargin {
    type Value = Property<f32>;
}

/// Border size
pub struct BorderSize;
impl TypeMapKey for BorderSize {
    type Value = Property<f32>;
}

/// Scroll bar margin (start)
pub struct ScrollBarStartMargin;
impl TypeMapKey for ScrollBarStartMargin {
    type Value = Property<f32>;
}

/// Scroll bar margin (end)
pub struct ScrollBarEndMargin;
impl TypeMapKey for ScrollBarEndMargin {
    type Value = Property<f32>;
}

/// Scroll bar margin (side)
pub struct ScrollBarSideMargin;
impl TypeMapKey for ScrollBarSideMargin {
    type Value = Property<f32>;
}

/// Scroll bar minimum thumb size
pub struct ScrollBarMinThumbSize;
impl TypeMapKey for ScrollBarMinThumbSize {
    type Value = Property<f32>;
}

/// Progress bar margin (start)
pub struct ProgressBarStartMargin;
impl TypeMapKey for ProgressBarStartMargin {
    type Value = Property<f32>;
}

/// Progress bar margin (end)
pub struct ProgressBarEndMargin;
impl TypeMapKey for ProgressBarEndMargin {
    type Value = Property<f32>;
}

/// Progress bar margin (side)
pub struct ProgressBarSideMargin;
impl TypeMapKey for ProgressBarSideMargin {
    type Value = Property<f32>;
}

/// Progress bar minimum size
pub struct ProgressBarMinSize;
impl TypeMapKey for ProgressBarMinSize {
    type Value = Property<f32>;
}
