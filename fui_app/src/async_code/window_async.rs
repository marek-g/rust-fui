use crate::WindowOptions;

#[derive(Clone)]
pub struct WindowAsync {
    window_options: WindowOptions,
}

impl WindowAsync {
    pub fn new(window_options: WindowOptions) -> Self {
        Self { window_options }
    }
}
