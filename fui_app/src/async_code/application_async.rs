use crate::{
    Application, DrawingContext, VMDispatcher, WindowGUIThreadData, WindowId, WindowManagerAsync,
    WindowOptions,
};
use anyhow::Result;
use fui_core::{register_current_thread_dispatcher, ViewModel};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;
use tokio::sync::oneshot;

thread_local! {
    pub static APPLICATION_CONTEXT: RefCell<Option<ApplicationContext>> = RefCell::new(None);
}

pub struct ApplicationContext {
    pub drawing_context: Rc<RefCell<DrawingContext>>,
    pub next_window_id: WindowId,
    pub windows: HashMap<WindowId, WindowGUIThreadData>,
}

pub struct ApplicationAsync {
    thread_join_handle: Option<JoinHandle<()>>,
    window_manager: Rc<RefCell<WindowManagerAsync>>,
}

impl ApplicationAsync {
    pub async fn new(title: &'static str) -> Result<Self> {
        register_current_thread_dispatcher(Box::new(VMDispatcher()));

        let (init_tx, init_rx) = oneshot::channel();

        let thread_join_handle = std::thread::Builder::new()
            .name("GUI".to_string())
            .spawn(move || {
                let app = fui_system::Application::new(
                    fui_system::ApplicationOptions::new()
                        .with_title(title)
                        .with_opengl_share_contexts(true)
                        .with_opengl_stencil_bits(8),
                )
                .unwrap();

                register_current_thread_dispatcher(Box::new(crate::gui_dispatcher::GUIDispatcher(
                    app.get_dispatcher(),
                )));

                let drawing_context = Rc::new(RefCell::new(DrawingContext::new().unwrap()));

                APPLICATION_CONTEXT.with(move |context| {
                    *context.borrow_mut() = Some(ApplicationContext {
                        drawing_context,
                        next_window_id: 1,
                        windows: HashMap::new(),
                    })
                });

                init_tx.send(()).unwrap();

                println!("Running qt thread: {:?}", thread::current().id());

                app.message_loop();
            })
            .unwrap();

        init_rx.await?;

        Ok(Self {
            thread_join_handle: Some(thread_join_handle),
            window_manager: Rc::new(RefCell::new(WindowManagerAsync::new()?)),
        })
    }

    pub fn get_window_manager(&self) -> Rc<RefCell<WindowManagerAsync>> {
        self.window_manager.clone()
    }

    pub fn run(&mut self) -> Result<()> {
        if let Some(handle) = self.thread_join_handle.take() {
            handle.join().unwrap();
        }
        Ok(())
    }
}
