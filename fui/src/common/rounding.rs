pub fn round_layout_value(value: f32, dpi_scale: f32) -> f32 {
    if (dpi_scale - 1.0f32).abs() < 0.0000015f32 {
        value.round()
    } else {
        let mut new_value = (value * dpi_scale).round() / dpi_scale;
        if new_value.is_infinite()
            || new_value.is_nan()
            || (std::f32::MAX - new_value) < 0.0000015f32
        {
            new_value = value;
        }
        new_value
    }
}
