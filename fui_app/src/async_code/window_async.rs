use crate::async_code::application_async::APPLICATION_CONTEXT;
use crate::{DrawingContext, WindowOptions};
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

///
/// Window data available only from the GUI thread.
///
pub struct WindowGUIThreadData {
    system_window: fui_system::Window,
    gl_context_data: Option<GlContextData>,
}

///
/// Window data available only from the VM (View Models) thread.
///
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
        // Run code on the GUI Thread
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

            let window_gui_thread_data = WindowGUIThreadData {
                system_window: native_window,
                gl_context_data: None,
            };

            let window_id = APPLICATION_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let mut app_context = context.as_mut().unwrap();

                let window_id = app_context.next_window_id;
                app_context
                    .windows
                    .insert(window_id, window_gui_thread_data);

                app_context.next_window_id += 1;

                window_id
            });

            Self::setup_window_events(window_id, &drawing_context);

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

    pub fn downgrade(&self) -> WindowWeakAsync {
        WindowWeakAsync {
            data: Rc::downgrade(&self.data),
        }
    }

    pub fn set_vm<V: ViewModel>(&self, view_model: Rc<RefCell<V>>) {
        let new_view = ViewModel::create_view(&view_model);

        let mut window_data = self.data.borrow_mut();
        if let Some(view) = window_data.view.take() {
            window_data.remove_layer(&view);
        }
        window_data.add_layer(new_view.clone());
        window_data.view.replace(new_view);
    }

    pub fn get_window_service(&self) -> Rc<RefCell<dyn fui_core::WindowService + 'static>> {
        let service: Rc<RefCell<dyn fui_core::WindowService + 'static>> = self.data.clone();
        service
    }

    fn setup_window_events(window_id: WindowId, drawing_context: &Rc<RefCell<DrawingContext>>) {
        APPLICATION_CONTEXT.with(move |context| {
            let mut context = context.borrow_mut();
            let mut app_context = context.as_mut().unwrap();

            app_context
                .func_gui2vm_thread_tx
                .send(Box::new(|| {
                    println!("Hello From Another thread: {:?}", thread::current().id());
                }) as Box<dyn 'static + Send + FnOnce()>);

            if let Some(window_data) = app_context.windows.get_mut(&window_id) {
                window_data.system_window.on_paint_gl({
                    let drawing_context_clone = drawing_context.clone();
                    let mut initialized = false;

                    move || {
                        let drawing_context_clone = drawing_context_clone.clone();
                        APPLICATION_CONTEXT.with(move |context| {
                            let mut context = context.borrow_mut();
                            let mut app_context = context.as_mut().unwrap();
                            if let Some(window_data) = app_context.windows.get_mut(&window_id) {
                                let mut drawing_context = drawing_context_clone.borrow_mut();

                                if !initialized {
                                    window_data.gl_context_data =
                                        Some(drawing_context.device.init_context(|symbol| {
                                            window_data
                                                .system_window
                                                .get_opengl_proc_address(symbol)
                                                .unwrap()
                                        }));
                                    initialized = true;
                                }

                                let width = window_data.system_window.get_width();
                                let height = window_data.system_window.get_height();
                                /*if width > 0 && height > 0 {
                                    Self::update_min_window_size(
                                        window_data,
                                        &mut drawing_context,
                                        0,
                                    );

                                    Self::render(
                                        window_data,
                                        &mut drawing_context,
                                        width as u32,
                                        height as u32,
                                        0,
                                    );
                                }*/
                            }
                        });
                    }
                });

                window_data.system_window.on_event({
                    let drawing_context_clone = drawing_context.clone();

                    move |event| {
                        let drawing_context_clone = drawing_context_clone.clone();
                        APPLICATION_CONTEXT.with(move |context| {
                            let mut context = context.borrow_mut();
                            let mut app_context = context.as_mut().unwrap();
                            if let Some(window_data) = app_context.windows.get_mut(&window_id) {
                                let system_window = &mut window_data.system_window;
                                let mut drawing_context = drawing_context_clone.borrow_mut();

                                let width = system_window.get_width();
                                let height = system_window.get_height();
                                /*let mut fui_drawing_context = FuiDrawingContext::new(
                                    (width as u16, height as u16),
                                    &mut drawing_context,
                                    0,
                                );

                                // events go to the window's root control
                                let root_control = window_data.root_control.clone();
                                window_data.event_processor.handle_event(
                                    &root_control,
                                    &mut fui_drawing_context,
                                    &input_event,
                                );*/

                                true
                            } else {
                                false
                            }
                        })
                    }
                });
            }
        });
    }
}

impl Drop for WindowVMThreadData {
    fn drop(&mut self) {
        let window_id = self.id;
        fui_system::Application::post_func(move || {
            APPLICATION_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let mut app_context = context.as_mut().unwrap();
                app_context.windows.remove(&window_id);
            });
        });
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
        let window_id = self.id;
        fui_system::Application::post_func(move || {
            APPLICATION_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let mut app_context = context.as_mut().unwrap();
                if let Some(window) = app_context.windows.get_mut(&window_id) {
                    window.system_window.update();
                }
            });
        });
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
