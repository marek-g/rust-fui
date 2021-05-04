pub struct QSurfaceFormat;

impl QSurfaceFormat {
    pub fn set_default(stencil_bits: i32) {
        unsafe {
            crate::platform::qt::qt_wrapper::QSurfaceFormat_setDefault(stencil_bits);
        }
    }
}
