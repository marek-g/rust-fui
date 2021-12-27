use fui_core::*;

use anyhow::Result;

use std::cell::RefCell;
use std::rc::Rc;

use crate::{DrawingContext, WindowOptions};

pub struct Application {
    app: fui_system::Application,
    drawing_context: Rc<RefCell<DrawingContext>>,
    windows: Vec<crate::Window>,
}

impl Application {
    pub fn new(title: &'static str) -> Result<Self> {
        let app = fui_system::Application::new(
            fui_system::ApplicationOptions::new()
                .with_title(title)
                .with_opengl_share_contexts(true)
                .with_opengl_stencil_bits(8),
        )?;

        register_current_thread_dispatcher(Box::new(crate::dispatcher::Dispatcher(
            app.get_dispatcher(),
        )));

        let drawing_context = Rc::new(RefCell::new(DrawingContext::new()?));

        Ok(Self {
            app,
            drawing_context,
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
        let mut window = crate::Window::new(window_options);
        window.create(&self.drawing_context)?;
        self.windows.push(window.clone());
        Ok(window)
    }

    pub fn run(&mut self) -> Result<()> {
        self.app.message_loop();
        Ok(())
    }

    #[cfg(feature = "async")]
    pub fn run_async(&mut self) {}

    pub fn exit() {
        fui_system::Application::exit(0);
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        // It is important to drop windows before drawing_context!
        // Windows cleanup graphics resources and drawing context drops graphics device.
        self.windows.clear();
    }
}
