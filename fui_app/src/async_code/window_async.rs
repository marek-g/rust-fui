use crate::async_code::application_async::APPLICATION_CONTEXT;
use crate::{GlWindow, WindowOptions};
use anyhow::Result;
use fui_core::ControlObject;
use fui_core::ViewModel;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::thread;
use tokio::sync::oneshot;

pub type WindowId = i64;

struct WindowData {
    id: WindowId,
    view: Option<Rc<RefCell<dyn ControlObject>>>,
    services: Option<Rc<RefCell<fui_core::Services>>>,
}

#[derive(Clone)]
pub struct WindowAsync {
    data: Rc<RefCell<WindowData>>,
}

impl WindowAsync {
    pub async fn create(window_options: WindowOptions) -> Result<Self> {
        let (tx, rx) = oneshot::channel::<WindowId>();

        fui_system::Application::post_func(move || {
            println!("Function Thread: {:?}", thread::current().id());
            println!("Function posted from another thread!");

            println!("{:?}", window_options.title);

            let mut native_window = fui_system::Window::new(None).unwrap();
            native_window.set_title(&window_options.title).unwrap();
            native_window.resize(window_options.width, window_options.height);
            native_window.set_visible(window_options.visible).unwrap();
            if window_options.icon.len() > 0 {
                let icon = fui_system::Icon::from_data(&window_options.icon).unwrap();
                native_window.set_icon(&icon).unwrap();
            }

            let drawing_context = APPLICATION_CONTEXT.with(|context| {
                context
                    .borrow_mut()
                    .as_ref()
                    .unwrap()
                    .drawing_context
                    .clone()
            });

            //setup_window_events(&native_window, &drawing_context);

            let core_window = fui_core::Window::new(GlWindow::new(native_window));
            let window_id = APPLICATION_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let mut app_context = context.as_mut().unwrap();

                let window_id = app_context.next_window_id;
                app_context.core_windows.insert(window_id, core_window);

                app_context.next_window_id += 1;

                window_id
            });

            tx.send(window_id);
        });

        let window_id = rx.await?;

        let window_data_rc = Rc::new(RefCell::new(WindowData {
            id: window_id,
            view: None,
            services: None,
        }));

        /*let window_service_rc: Rc<RefCell<dyn WindowService>> = window_data_rc.clone();
        let services = Rc::new(RefCell::new(fui_core::Services::new(&window_service_rc)));
        window_data_rc
            .borrow_mut()
            .core_window
            .get_root_control()
            .borrow_mut()
            .get_context_mut()
            .set_services(Some(Rc::downgrade(&services)));
        window_data_rc.borrow_mut().services = Some(services);*/

        Ok(WindowAsync {
            data: window_data_rc,
        })
    }

    pub fn set_vm<V: ViewModel>(&self, view_model: Rc<RefCell<V>>) {}

    pub fn downgrade(&self) -> WindowWeakAsync {
        WindowWeakAsync {
            data: Rc::downgrade(&self.data),
        }
    }
}

impl Drop for WindowData {
    fn drop(&mut self) {
        todo!("Remove item from ApplicationContext.core_windows on GUI thread.")
    }
}

pub struct WindowWeakAsync {
    data: Weak<RefCell<WindowData>>,
}

impl WindowWeakAsync {
    pub fn upgrade(&self) -> Option<WindowAsync> {
        self.data.upgrade().map(|d| WindowAsync { data: d })
    }
}
