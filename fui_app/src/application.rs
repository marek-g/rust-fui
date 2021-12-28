use crate::WindowManager;
use anyhow::Result;
use fui_core::register_current_thread_dispatcher;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Application {
    app: fui_system::Application,
    window_manager: Rc<RefCell<WindowManager>>,
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

        Ok(Self {
            app,
            window_manager: Rc::new(RefCell::new(WindowManager::new()?)),
        })
    }

    pub fn get_window_manager(&self) -> Rc<RefCell<WindowManager>> {
        self.window_manager.clone()
    }

    pub fn run(&mut self) -> Result<()> {
        self.app.message_loop();
        Ok(())
    }

    pub fn exit() {
        fui_system::Application::exit(0);
    }
}
