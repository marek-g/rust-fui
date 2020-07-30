/// size in scaled pixels (pixel size defined by user preferences)
/// if not specified otherwise this is the default unit to measure things
pub struct Sp(f32);

/// size in millimeters
pub struct Mm(f32);

/// size in range 0..1 - covers whole window area
pub struct Normal(f32);
