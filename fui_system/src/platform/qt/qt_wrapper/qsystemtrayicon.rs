use crate::platform::qt::qt_wrapper::{QIcon, QMenu, QString};
use crate::FUISystemError;

pub struct QSystemTrayIcon {
    pub this: *mut ::std::os::raw::c_void,
}

impl QSystemTrayIcon {
    pub fn new() -> Result<Self, FUISystemError> {
        unsafe {
            let this = crate::platform::qt::qt_wrapper::QSystemTrayIcon_new();
            if this.is_null() {
                return Err(FUISystemError::OutOfMemory);
            }

            Ok(Self { this })
        }
    }

    // keeps reference to QMenu, doesn't take ownership
    pub fn set_context_menu(&mut self, menu: &mut QMenu) {
        unsafe {
            crate::platform::qt::qt_wrapper::QSystemTrayIcon_setContextMenu(self.this, menu.this);
        }
    }

    pub fn set_icon(&mut self, icon: &QIcon) {
        unsafe {
            crate::platform::qt::qt_wrapper::QSystemTrayIcon_setIcon(self.this, icon.this);
        }
    }

    pub fn set_tool_tip(&mut self, tip: &QString) {
        unsafe {
            crate::platform::qt::qt_wrapper::QSystemTrayIcon_setToolTip(self.this, tip.this);
        }
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        unsafe {
            crate::platform::qt::qt_wrapper::QSystemTrayIcon_setVisible(
                self.this,
                if is_visible { 1 } else { 0 },
            );
        }
    }

    pub fn show_message(&mut self, title: &QString, message: &QString, icon: i32, timeout: i32) {
        unsafe {
            crate::platform::qt::qt_wrapper::QSystemTrayIcon_showMessage(
                self.this,
                title.this,
                message.this,
                icon,
                timeout,
            );
        }
    }

    pub fn show_message2(
        &mut self,
        title: &QString,
        message: &QString,
        icon: &QIcon,
        timeout: i32,
    ) {
        unsafe {
            crate::platform::qt::qt_wrapper::QSystemTrayIcon_showMessage2(
                self.this,
                title.this,
                message.this,
                icon.this,
                timeout,
            );
        }
    }
}

impl Drop for QSystemTrayIcon {
    fn drop(&mut self) {
        unsafe {
            crate::platform::qt::qt_wrapper::QSystemTrayIcon_delete(self.this);
        }
    }
}
