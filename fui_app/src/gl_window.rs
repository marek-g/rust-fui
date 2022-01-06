use drawing_gl::GlContextData;

pub struct GlWindow {
    pub window: fui_system::Window,
    pub gl_context_data: Option<GlContextData>,
}

impl GlWindow {
    pub fn new(window: fui_system::Window) -> Self {
        Self {
            window,
            gl_context_data: None,
        }
    }

    pub fn repaint(&mut self) {
        self.window.update();
    }
}
