use std::{cell::RefCell, rc::Rc, rc::Weak};
use crate::WindowService;

pub struct Services {
    window_service: Weak<RefCell<dyn WindowService>>, 
}

impl Services {
    pub fn new(window_service: &Rc<RefCell<dyn WindowService>>) -> Self {
        Self {
            window_service: Rc::downgrade(window_service)
        }
    }

    pub fn get_window_service(&self) -> Option<Rc<RefCell<dyn WindowService>>> {
        self.window_service.upgrade()
    }
}
