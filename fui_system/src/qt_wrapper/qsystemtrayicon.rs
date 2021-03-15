use crate::qt_wrapper::{QIcon, QString};

pub struct QSystemTrayIcon {
    pub this: *mut ::std::os::raw::c_void,
}

impl QSystemTrayIcon {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let this = crate::qt_wrapper::QSystemTrayIcon_new();
            if this.is_null() {
                return Err(());
            }

            Ok(Self { this })
        }
    }

    pub fn set_icon(&mut self, icon: &QIcon) -> Result<(), ()> {
        unsafe {
            crate::qt_wrapper::QSystemTrayIcon_setIcon(self.this, icon.this);
            Ok(())
        }
    }

    pub fn set_tool_tip(&mut self, tip: &QString) -> Result<(), ()> {
        unsafe {
            crate::qt_wrapper::QSystemTrayIcon_setToolTip(self.this, tip.this);
            Ok(())
        }
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        unsafe {
            crate::qt_wrapper::QSystemTrayIcon_setVisible(
                self.this,
                if is_visible { 1 } else { 0 },
            );
        }
    }

    pub fn show_message(
        &mut self,
        title: &QString,
        message: &QString,
        icon: i32,
        timeout: i32,
    ) -> Result<(), ()> {
        unsafe {
            crate::qt_wrapper::QSystemTrayIcon_showMessage(
                self.this,
                title.this,
                message.this,
                icon,
                timeout,
            );
            Ok(())
        }
    }

    pub fn show_message2(
        &mut self,
        title: &QString,
        message: &QString,
        icon: &QIcon,
        timeout: i32,
    ) -> Result<(), ()> {
        unsafe {
            crate::qt_wrapper::QSystemTrayIcon_showMessage2(
                self.this,
                title.this,
                message.this,
                icon.this,
                timeout,
            );
            Ok(())
        }
    }
}

impl Drop for QSystemTrayIcon {
    fn drop(&mut self) {
        unsafe {
            crate::qt_wrapper::QSystemTrayIcon_delete(self.this);
        }
    }
}
