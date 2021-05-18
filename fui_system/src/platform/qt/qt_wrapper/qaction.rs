use crate::common::callback_helper::RawCallback;
use crate::platform::qt::qt_wrapper::{QIcon, QSlot, QString};
use crate::FUISystemError;

pub struct QAction {
    pub this: *mut ::std::os::raw::c_void,
    pub is_owned: bool,
}

impl QAction {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QAction_new();
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(Self {
                this,
                is_owned: true,
            })
        }
    }

    pub fn set_text(&mut self, text: &QString) {
        unsafe {
            crate::platform::qt::qt_wrapper::QAction_setText(self.this, text.this);
        }
    }

    pub fn set_shortcut(&mut self, text: &QString) {
        unsafe {
            crate::platform::qt::qt_wrapper::QAction_setShortcut(self.this, text.this);
        }
    }

    pub fn set_icon(&mut self, icon: &QIcon) {
        unsafe {
            crate::platform::qt::qt_wrapper::QAction_setIcon(self.this, icon.this);
        }
    }

    pub fn connect_triggered(
        &mut self,
        raw_callback: RawCallback,
    ) -> Result<QSlot, FUISystemError> {
        let slot = QSlot::new(raw_callback)?;
        unsafe {
            crate::platform::qt::qt_wrapper::QAction_connectTriggered(self.this, slot.this);
        }
        Ok(slot)
    }
}

impl Drop for QAction {
    fn drop(&mut self) {
        if self.is_owned {
            unsafe {
                crate::platform::qt::qt_wrapper::QAction_delete(self.this);
            }
        }
    }
}
