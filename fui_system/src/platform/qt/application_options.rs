///
/// Global application options.
///
pub struct ApplicationOptions {
    pub title: String,

    pub opengl_share_contexts: bool,
    pub opengl_stencil_bits: i32,
}

///
/// Builder for ApplicationOptions.
///
pub struct ApplicationOptionsBuilder {
    title: String,

    opengl_share_contexts: bool,
    opengl_stencil_bits: i32,
}

impl ApplicationOptionsBuilder {
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
    pub fn with_title(self, title: &str) -> Self {
        Self {
            title: title.to_string(),
            opengl_share_contexts: self.opengl_share_contexts,
            opengl_stencil_bits: self.opengl_stencil_bits,
        }
    }

    ///
    /// Enables automatically created shared contexts between windows.
    ///
    pub fn with_opengl_share_contexts(self, share: bool) -> Self {
        Self {
            title: self.title,
            opengl_share_contexts: share,
            opengl_stencil_bits: self.opengl_stencil_bits,
        }
    }

    ///
    /// Sets bits of the the stencil buffer.
    /// 0 - disables stencil buffer, 8 - standard stencil buffer.
    ///
    pub fn with_opengl_stencil_bits(self, bits: i32) -> Self {
        Self {
            title: self.title,
            opengl_share_contexts: self.opengl_share_contexts,
            opengl_stencil_bits: bits,
        }
    }

    ///
    /// Creates ApplicationOptions object.
    ///
    pub fn build(self) -> ApplicationOptions {
        ApplicationOptions {
            title: self.title,
            opengl_share_contexts: self.opengl_share_contexts,
            opengl_stencil_bits: self.opengl_stencil_bits,
        }
    }
}
