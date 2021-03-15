use crate::qt_wrapper::{QIcon, QPixmap, QString, QSystemTrayIcon};
use crate::TrayError;

pub enum SystemMessageIcon<'a> {
    NoIcon,
    Information,
    Warning,
    Critical,
    Custom(&'a [u8]),
}

pub struct SystemTray {
    qtray: QSystemTrayIcon,
}

impl SystemTray {
    pub fn new() -> Result<Self, ()> {
        Ok(SystemTray {
            qtray: QSystemTrayIcon::new()?,
        })
    }

    pub fn set_icon(&mut self, data: &[u8]) -> Result<(), ()> {
        self.qtray.set_icon(&Self::create_icon(data)?)?;
        Ok(())
    }

    pub fn set_tool_tip(&mut self, tip: &str) -> Result<(), ()> {
        let tip = QString::from_str(tip)?;
        self.qtray.set_tool_tip(&tip)?;
        Ok(())
    }

    pub fn set_visible(&mut self, visible: bool) -> Result<(), TrayError> {
        self.qtray.set_visible(visible);
        Ok(())
    }

    pub fn show_message(
        &mut self,
        title: &str,
        message: &str,
        icon: SystemMessageIcon,
        timeout: i32,
    ) -> Result<(), ()> {
        let title = QString::from_str(title)?;
        let message = QString::from_str(message)?;

        match icon {
            SystemMessageIcon::NoIcon => {
                self.qtray.show_message(&title, &message, 0, timeout)?;
            }

            SystemMessageIcon::Information => {
                self.qtray.show_message(&title, &message, 1, timeout)?;
            }
            SystemMessageIcon::Warning => {
                self.qtray.show_message(&title, &message, 2, timeout)?;
            }

            SystemMessageIcon::Critical => {
                self.qtray.show_message(&title, &message, 3, timeout)?;
            }

            SystemMessageIcon::Custom(data) => {
                self.qtray
                    .show_message2(&title, &message, &Self::create_icon(data)?, timeout)?;
            }
        }

        Ok(())
    }

    fn create_icon(data: &[u8]) -> Result<QIcon, ()> {
        let pixmap = QPixmap::from_data(&data)?;

        let mut icon = QIcon::new()?;
        icon.add_pixmap(&pixmap);

        Ok(icon)
    }
}
