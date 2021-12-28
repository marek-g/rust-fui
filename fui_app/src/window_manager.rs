use crate::{DrawingContext, WindowOptions};
use anyhow::Result;
use fui_core::ViewModel;
use std::cell::RefCell;
use std::rc::Rc;

pub struct WindowManager {
    drawing_context: Rc<RefCell<DrawingContext>>,
    windows: Vec<crate::Window>,
}

impl WindowManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            drawing_context: Rc::new(RefCell::new(DrawingContext::new()?)),
            windows: Vec::new(),
        })
    }

    pub fn add_window<V: ViewModel>(
        &mut self,
        window_options: WindowOptions,
        view_model: Rc<RefCell<V>>,
    ) -> Result<crate::Window> {
        let window = self.create_window(window_options)?;
        window.set_vm(view_model);
        Ok(window)
    }

    pub fn create_window(&mut self, window_options: WindowOptions) -> Result<crate::Window> {
        let window = crate::Window::create(window_options, &self.drawing_context)?;
        self.windows.push(window.clone());
        Ok(window)
    }
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        // It is important to drop windows before drawing_context!
        // Windows cleanup graphics resources and drawing context drops graphics device.
        self.windows.clear();
    }
}
