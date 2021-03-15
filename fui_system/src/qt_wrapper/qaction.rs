use crate::qt_wrapper::QString;

pub struct QAction {
    pub this: *mut ::std::os::raw::c_void,
    pub is_owned: bool,
}

impl QAction {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let this = crate::qt_wrapper::QAction_new();
            if this.is_null() {
                return Err(());
            }

            Ok(Self {
                this,
                is_owned: true,
            })
        }
    }

    pub fn set_text(&mut self, text: &QString) -> Result<(), ()> {
        unsafe {
            crate::qt_wrapper::QAction_setText(self.this, text.this);
            Ok(())
        }
    }
}

impl Drop for QAction {
    fn drop(&mut self) {
        if self.is_owned {
            unsafe {
                crate::qt_wrapper::QAction_delete(self.this);
            }
        }
    }
}
