use crate::{Application, DrawingContext, WindowManagerAsync, WindowOptions};
use anyhow::Result;
use fui_core::{register_current_thread_dispatcher, ViewModel};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;
use tokio::sync::oneshot;

thread_local! {
    static APPLICATION: RefCell<Option<ApplicationContext>> = RefCell::new(None);
}

struct ApplicationContext {
    drawing_context: Rc<RefCell<DrawingContext>>,
}

pub struct ApplicationAsync {
    thread_join_handle: Option<JoinHandle<()>>,
    window_manager: Rc<RefCell<WindowManagerAsync>>,
}

impl ApplicationAsync {
    pub async fn new(title: &'static str) -> Result<Self> {
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

                register_current_thread_dispatcher(Box::new(crate::dispatcher::Dispatcher(
                    app.get_dispatcher(),
                )));

                let drawing_context = Rc::new(RefCell::new(DrawingContext::new().unwrap()));

                APPLICATION.with(move |context| {
                    *context.borrow_mut() = Some(ApplicationContext { drawing_context })
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

    /*pub async fn add_window<V: ViewModel>(
        &mut self,
        window_options: WindowOptions,
        view_model: Rc<RefCell<V>>,
    ) -> Result<crate::WindowAsync> {
        let window = self.create_window(window_options).await?;

        //window.set_vm(view_model);
        Ok(window)
    }

    pub async fn create_window(
        &mut self,
        window_options: WindowOptions,
    ) -> Result<crate::WindowAsync> {
        let mut window = crate::WindowAsync::new(window_options.clone());

        fui_system::Application::post_func(move || {
            println!("Function Thread: {:?}", thread::current().id());
            println!("Function posted from another thread!");

            println!("{:?}", window_options.title);

            let mut window = crate::Window::new(window_options);

            let drawing_context = APPLICATION.with(|context| {
                context
                    .borrow_mut()
                    .as_ref()
                    .unwrap()
                    .drawing_context
                    .clone()
            });

            window.create(&drawing_context).unwrap();
            //self.windows.push(window.clone());

            //let mut native_window = fui_system::Window::new(None).unwrap();
            //native_window.set_visible(true);
            Box::leak(Box::new(window));
        });

        self.windows.push(window.clone());
        Ok(window)
    }*/

    pub fn run(&mut self) -> Result<()> {
        if let Some(handle) = self.thread_join_handle.take() {
            handle.join().unwrap();
        }
        Ok(())
    }
}
