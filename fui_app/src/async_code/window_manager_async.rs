use crate::{DrawingContext, WindowOptions};
use anyhow::Result;
use fui_core::ViewModel;
use std::cell::RefCell;
use std::rc::Rc;

pub struct WindowManagerAsync {
    windows: Vec<crate::WindowAsync>,
}

impl WindowManagerAsync {
    pub fn new() -> Result<Self> {
        Ok(Self {
            windows: Vec::new(),
        })
    }

    pub async fn add_window<V: ViewModel>(
        &mut self,
        window_options: WindowOptions,
        view_model: Rc<RefCell<V>>,
    ) -> Result<crate::WindowAsync> {
        let window = self.create_window(window_options).await?;
        window.set_vm(view_model);
        Ok(window)
    }

    pub async fn create_window(
        &mut self,
        window_options: WindowOptions,
    ) -> Result<crate::WindowAsync> {
        let mut window = crate::WindowAsync::new(window_options);
        window.create().await?;
        self.windows.push(window.clone());
        Ok(window)
    }
}

impl Drop for WindowManagerAsync {
    fn drop(&mut self) {
        // It is important to drop windows before drawing_context!
        // Windows cleanup graphics resources and drawing context drops graphics device.
        self.windows.clear();
    }
}
