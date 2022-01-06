use crate::async_code::application_async::APPLICATION_CONTEXT;
use crate::WindowOptions;
use anyhow::Result;
use drawing_gl::GlContextData;
use fui_core::{Children, Grid, ViewContext};
use fui_core::{ControlObject, EventProcessor, ObservableVec};
use fui_core::{ViewModel, WindowService};
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::thread;
use tokio::sync::oneshot;
use typemap::TypeMap;

pub type WindowId = i64;

pub struct WindowGUIThreadData {
    system_window: fui_system::Window,
    gl_context_data: Option<GlContextData>,
}

struct WindowVMThreadData {
    id: WindowId,

    event_processor: EventProcessor,
    root_control: Rc<RefCell<dyn ControlObject>>,
    view: Option<Rc<RefCell<dyn ControlObject>>>,
    services: Option<Rc<RefCell<fui_core::Services>>>,

    control_layers: ObservableVec<Rc<RefCell<dyn ControlObject>>>,
}

#[derive(Clone)]
pub struct WindowAsync {
    data: Rc<RefCell<WindowVMThreadData>>,
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

            let thread_window = WindowGUIThreadData {
                system_window: native_window,
                gl_context_data: None,
            };

            let window_id = APPLICATION_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let mut app_context = context.as_mut().unwrap();

                let window_id = app_context.next_window_id;
                app_context.windows.insert(window_id, thread_window);

                app_context.next_window_id += 1;

                window_id
            });

            tx.send(window_id);
        });

        let window_id = rx.await?;

        let control_layers = ObservableVec::<Rc<RefCell<dyn ControlObject>>>::new();

        let content = ui!(
            Grid {
                &control_layers,
            }
        );

        let window_data_rc = Rc::new(RefCell::new(WindowVMThreadData {
            id: window_id,
            event_processor: EventProcessor::new(),
            root_control: content,
            view: None,
            services: None,
            control_layers,
        }));

        let window_service_rc: Rc<RefCell<dyn WindowService>> = window_data_rc.clone();
        let services = Rc::new(RefCell::new(fui_core::Services::new(&window_service_rc)));
        window_data_rc
            .borrow_mut()
            .root_control
            .borrow_mut()
            .get_context_mut()
            .set_services(Some(Rc::downgrade(&services)));
        window_data_rc.borrow_mut().services = Some(services);

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

impl Drop for WindowVMThreadData {
    fn drop(&mut self) {
        todo!("Remove item from ApplicationContext.core_windows on GUI thread.")
    }
}

impl fui_core::WindowService for WindowVMThreadData {
    fn add_layer(&mut self, control: Rc<RefCell<dyn ControlObject>>) {
        self.control_layers.push(control);
    }

    fn remove_layer(&mut self, control: &Rc<RefCell<dyn ControlObject>>) {
        self.control_layers
            .remove_filter(|el| Rc::ptr_eq(el, control));
    }

    fn repaint(&mut self) {
        // TODO:
        //self.system_window.update();
    }
}

pub struct WindowWeakAsync {
    data: Weak<RefCell<WindowVMThreadData>>,
}

impl WindowWeakAsync {
    pub fn upgrade(&self) -> Option<WindowAsync> {
        self.data.upgrade().map(|d| WindowAsync { data: d })
    }
}
