use crate::{DrawingContext, Window, WindowId, WindowOptions};
use anyhow::Result;
use fui_core::ViewModel;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct WindowManager {
    windows: HashMap<WindowId, crate::Window>,
}

impl WindowManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            windows: HashMap::new(),
        })
    }

    pub async fn add_window<V: ViewModel>(
        &mut self,
        window_options: WindowOptions,
        view_model: Rc<RefCell<V>>,
    ) -> Result<crate::Window> {
        let window = self.create_window(window_options).await?;
        window.set_vm(view_model);
        Ok(window)
    }

    pub async fn create_window(&mut self, window_options: WindowOptions) -> Result<crate::Window> {
        let mut window = crate::Window::create(window_options).await?;
        self.windows.insert(window.get_id(), window.clone());
        Ok(window)
    }

    pub fn get_window_mut(&mut self, window_id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&window_id)
    }
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        // It is important to drop windows before drawing_context!
        // Windows cleanup graphics resources and drawing context drops graphics device.
        self.windows.clear();
    }
}
