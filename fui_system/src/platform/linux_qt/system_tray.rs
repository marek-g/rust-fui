use crate::qt_wrapper::{QIcon, QMenu, QPixmap, QString, QSystemTrayIcon};
use crate::TrayError;
use fui_core::MenuItem;

pub enum SystemMessageIcon<'a> {
    NoIcon,
    Information,
    Warning,
    Critical,
    Custom(&'a [u8]),
}

pub struct SystemTray {
    qtray: QSystemTrayIcon,
    qmenu: Option<QMenu>,
}

impl SystemTray {
    pub fn new() -> Result<Self, ()> {
        Ok(SystemTray {
            qtray: QSystemTrayIcon::new()?,
            qmenu: None,
        })
    }

    pub fn set_icon(&mut self, data: &[u8]) -> Result<(), ()> {
        self.qtray.set_icon(&Self::create_icon(data)?)?;
        Ok(())
    }

    pub fn set_menu(&mut self, menu_items: &Vec<MenuItem>) -> Result<(), ()> {
        let mut qmenu = Self::qmenu_from_menu_items(menu_items)?;
        self.qtray.set_context_menu(&mut qmenu);
        self.qmenu = Some(qmenu);
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

    fn qmenu_from_menu_items(menu_items: &Vec<MenuItem>) -> Result<QMenu, ()> {
        unsafe {
            let mut qmenu = QMenu::new()?;

            SystemTray::qmenu_add_menu_items(&mut qmenu, menu_items);

            Ok(qmenu)
        }
    }

    fn qmenu_add_menu_items(mut qmenu: &mut QMenu, menu_items: &Vec<MenuItem>) -> Result<(), ()> {
        for menu_item in menu_items {
            Self::qmenu_add_menu_item(&mut qmenu, menu_item)?;
        }
        Ok(())
    }

    fn qmenu_add_menu_item(qmenu: &mut QMenu, menu_item: &MenuItem) -> Result<(), ()> {
        match menu_item {
            MenuItem::Separator => {
                qmenu.add_separator()?;
            }

            MenuItem::Text {
                text,
                shortcut,
                icon,
                callback,
                sub_items,
            } => {
                if sub_items.len() > 0 {
                    let mut qsubmenu = qmenu.add_menu(&QString::from_str(&text)?)?;
                    Self::qmenu_add_menu_items(&mut qsubmenu, sub_items)?;
                } else {
                    qmenu.add_action_text(&QString::from_str(&text)?)?;
                }
            }

            MenuItem::Custom { .. } => {}
        }

        Ok(())
    }
}
