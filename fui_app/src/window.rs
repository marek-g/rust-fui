use crate::GlWindow;
use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Window {
    pub core_window: Rc<RefCell<Option<Rc<RefCell<fui_core::Window<GlWindow>>>>>>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            core_window: Rc::new(RefCell::new(None)),
        }
    }

    pub fn set_core_window(&self, window: fui_core::Window<GlWindow>) {
        *self.core_window.borrow_mut() = Some(Rc::new(RefCell::new(window)));
    }

    pub fn get_core_window(&self) -> Option<Rc<RefCell<fui_core::Window<GlWindow>>>> {
        match self.core_window.borrow().as_ref() {
            None => None,
            Some(el) => Some(el.clone()),
        }
    }

    pub fn get_window_service(&self) -> Option<Rc<RefCell<dyn fui_core::WindowService + 'static>>> {
        match self.core_window.borrow().as_ref() {
            None => None,
            Some(el) => Some(el.clone()),
        }
    }
}
