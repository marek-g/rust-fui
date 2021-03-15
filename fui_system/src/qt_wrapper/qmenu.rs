use crate::qt_wrapper::{QAction, QString};

pub struct QMenu {
    pub this: *mut ::std::os::raw::c_void,
}

impl QMenu {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let this = crate::qt_wrapper::QMenu_new();
            if this.is_null() {
                return Err(());
            }

            Ok(Self { this })
        }
    }

    pub fn add_action_text(&mut self, text: &QString) -> Result<QAction, ()> {
        unsafe {
            let qaction_this = crate::qt_wrapper::QMenu_addAction_text(self.this, text.this);
            if qaction_this.is_null() {
                return Err(());
            }

            Ok(QAction {
                this: qaction_this,
                is_owned: false,
            })
        }
    }
}

impl Drop for QMenu {
    fn drop(&mut self) {
        unsafe {
            crate::qt_wrapper::QMenu_delete(self.this);
        }
    }
}
