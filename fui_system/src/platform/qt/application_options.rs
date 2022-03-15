///
/// Global application options.
///
pub struct ApplicationOptions {
    pub title: String,

    pub opengl_share_contexts: bool,
    pub opengl_stencil_bits: i32,
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationOptions {
    ///
    /// Creates new builder.
    ///
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            opengl_share_contexts: false,
            opengl_stencil_bits: 0,
        }
    }

    ///
    /// Sets the application title.
    ///
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    ///
    /// Enables automatically created shared contexts between windows.
    ///
    pub fn with_opengl_share_contexts(mut self, share: bool) -> Self {
        self.opengl_share_contexts = share;
        self
    }

    ///
    /// Sets bits of the the stencil buffer.
    /// 0 - disables stencil buffer, 8 - standard stencil buffer.
    ///
    pub fn with_opengl_stencil_bits(mut self, bits: i32) -> Self {
        self.opengl_stencil_bits = bits;
        self
    }
}
