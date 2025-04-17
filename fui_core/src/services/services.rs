use crate::WindowService;
use std::{cell::RefCell, rc::Rc, rc::Weak};

use super::FileDialogService;

pub struct Services {
    window_service: Weak<RefCell<dyn WindowService>>,
    file_dialog_service: Rc<dyn FileDialogService>,
}

impl Services {
    pub fn new(
        window_service: &Rc<RefCell<dyn WindowService>>,
        file_dialog_service: Rc<dyn FileDialogService>,
    ) -> Self {
        Self {
            window_service: Rc::downgrade(window_service),
            file_dialog_service,
        }
    }

    pub fn get_window_service(&self) -> Option<Rc<RefCell<dyn WindowService>>> {
        self.window_service.upgrade()
    }

    pub fn get_file_dialog_service(&self) -> Rc<dyn FileDialogService> {
        self.file_dialog_service.clone()
    }
}
