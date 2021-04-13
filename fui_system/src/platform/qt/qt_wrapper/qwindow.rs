use crate::platform::qt::qt_wrapper::QString;

pub struct QWindow {
    pub this: *mut ::std::os::raw::c_void,
}

impl QWindow {
    pub fn new(parent: Option<&mut QWindow>) -> Result<Self, ()> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QWindow_new(
                parent.map_or(0 as *mut ::std::os::raw::c_void, |p| p.this),
            );
            if this.is_null() {
                return Err(());
            }

            Ok(Self { this })
        }
    }

    pub fn set_title(&mut self, text: &QString) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_setTitle(self.this, text.this);
        }
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_setVisible(
                self.this,
                if is_visible { 1 } else { 0 },
            );
        }
    }
}

impl Drop for QWindow {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QWindow_delete(self.this);
        }
    }
}
