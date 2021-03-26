use crate::qt_wrapper::{QAction, QSlot, QString};

pub struct QMenu {
    pub this: *mut ::std::os::raw::c_void,
    pub is_owned: bool,
}

impl QMenu {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let this = crate::qt_wrapper::QMenu_new();
            if this.is_null() {
                return Err(());
            }

            Ok(Self {
                this,
                is_owned: true,
            })
        }
    }

    pub fn add_action_text(&mut self, text: &QString) -> Result<QAction, ()> {
        unsafe {
            let qaction_this = crate::qt_wrapper::QMenu_addAction_text(self.this, text.this);
            if qaction_this.is_null() {
                return Err(());
            }

            let mut action = QAction {
                this: qaction_this,
                is_owned: false,
            };

            action.set_text(text);

            Ok(action)
        }
    }

    pub fn add_separator(&mut self) -> Result<QAction, ()> {
        unsafe {
            let qaction_this = crate::qt_wrapper::QMenu_addSeparator(self.this);
            if qaction_this.is_null() {
                return Err(());
            }

            Ok(QAction {
                this: qaction_this,
                is_owned: false,
            })
        }
    }

    pub fn add_menu(&mut self, text: &QString) -> Result<QMenu, ()> {
        unsafe {
            let qmenu_this = crate::qt_wrapper::QMenu_addMenu(self.this, text.this);
            if qmenu_this.is_null() {
                return Err(());
            }

            Ok(QMenu {
                this: qmenu_this,
                is_owned: false,
            })
        }
    }
}

impl Drop for QMenu {
    fn drop(&mut self) {
        if self.is_owned {
            unsafe {
                crate::qt_wrapper::QMenu_delete(self.this);
            }
        }
    }
}
