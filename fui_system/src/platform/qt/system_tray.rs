use crate::common::callback_helper::RawCallback;
use crate::platform::qt::qt_wrapper::{QIcon, QMenu, QPixmap, QSlot, QString, QSystemTrayIcon};
use crate::{SystemMenuItem, TrayError};

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
    slots: Vec<QSlot>,
}

impl SystemTray {
    pub fn new() -> Result<Self, ()> {
        Ok(SystemTray {
            qtray: QSystemTrayIcon::new()?,
            qmenu: None,
            slots: Vec::new(),
        })
    }

    pub fn set_icon(&mut self, data: &[u8]) -> Result<(), ()> {
        self.qtray.set_icon(&Self::create_icon(data)?)?;
        Ok(())
    }

    pub fn set_menu(&mut self, menu_items: &Vec<SystemMenuItem>) -> Result<(), ()> {
        let (mut qmenu, slots) = Self::qmenu_from_menu_items(menu_items)?;
        self.qtray.set_context_menu(&mut qmenu);
        self.qmenu = Some(qmenu);
        self.slots = slots;
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

    fn qmenu_from_menu_items(menu_items: &Vec<SystemMenuItem>) -> Result<(QMenu, Vec<QSlot>), ()> {
        unsafe {
            let mut qmenu = QMenu::new()?;
            let mut slots = Vec::new();

            SystemTray::qmenu_add_menu_items(&mut qmenu, &mut slots, menu_items);

            Ok((qmenu, slots))
        }
    }

    fn qmenu_add_menu_items(
        mut qmenu: &mut QMenu,
        slots: &mut Vec<QSlot>,
        menu_items: &Vec<SystemMenuItem>,
    ) -> Result<(), ()> {
        for menu_item in menu_items {
            Self::qmenu_add_menu_item(&mut qmenu, slots, menu_item)?;
        }
        Ok(())
    }

    fn qmenu_add_menu_item(
        qmenu: &mut QMenu,
        slots: &mut Vec<QSlot>,
        menu_item: &SystemMenuItem,
    ) -> Result<(), ()> {
        match menu_item {
            SystemMenuItem::Separator => {
                qmenu.add_separator()?;
            }

            SystemMenuItem::Text {
                text,
                shortcut,
                //icon,
                callback,
                sub_items,
            } => {
                if sub_items.len() > 0 {
                    let mut qsubmenu = qmenu.add_menu(&QString::from_str(&text)?)?;
                    Self::qmenu_add_menu_items(&mut qsubmenu, slots, sub_items)?;
                } else {
                    let mut action = qmenu.add_action_text(&QString::from_str(&text)?)?;
                    if let Some(callback) = callback {
                        let slot = action.connect_triggered(callback)?;
                        slots.push(slot);
                    }
                }
            }
        }

        Ok(())
    }
}
