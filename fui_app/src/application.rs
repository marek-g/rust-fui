use fui_core::*;

use anyhow::Result;

use std::cell::RefCell;
use std::rc::Rc;

use crate::FuiDrawingContext;
use crate::GlWindow;
use crate::{DrawingContext, WindowOptions};
use drawing_gl::GlRenderTarget;
use rand::{thread_rng, Rng};

struct ApplicationRuntimeData {
    windows: Vec<crate::Window>,
    drawing_context: Rc<RefCell<DrawingContext>>,
}

pub struct Application {
    title: &'static str,
    windows_to_create: Vec<crate::Window>,
    runtime_data: Option<ApplicationRuntimeData>,
}

impl Application {
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            windows_to_create: Vec::new(),
            runtime_data: None,
        }
    }

    pub fn add_window<V: ViewModel>(
        &mut self,
        window_options: WindowOptions,
        view_model: Rc<RefCell<V>>,
    ) -> Result<crate::Window> {
        let mut window = self.create_window(window_options)?;
        window.set_vm(view_model);
        Ok(window)
    }

    pub fn create_window(&mut self, window_options: WindowOptions) -> Result<crate::Window> {
        match self.runtime_data {
            None => {
                let window = crate::Window::new(window_options);
                self.windows_to_create.push(window.clone());
                Ok(window)
            }

            Some(ref mut runtime_data) => {
                let mut window = crate::Window::new(window_options);
                window.create(&runtime_data.drawing_context);
                runtime_data.windows.push(window.clone());
                Ok(window)
            }
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let app = fui_system::Application::new(
            fui_system::ApplicationOptions::new()
                .with_title(self.title)
                .with_opengl_share_contexts(true)
                .with_opengl_stencil_bits(8),
        )?;

        register_current_thread_dispatcher(Box::new(crate::dispatcher::Dispatcher(
            app.get_dispatcher(),
        )));

        let drawing_context = Rc::new(RefCell::new(DrawingContext::new()?));

        let mut windows = Vec::new();
        for mut window in &mut self.windows_to_create {
            window.create(&drawing_context)?;
        }
        windows.append(&mut self.windows_to_create);

        let runtime_data = ApplicationRuntimeData {
            drawing_context,
            windows,
        };
        self.runtime_data = Some(runtime_data);

        app.message_loop();

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
        if let Some(ref mut runtime_data) = self.runtime_data {
            runtime_data.windows.clear();
        }
    }
}
