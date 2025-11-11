use crate::{AppFileDialog, APPLICATION_GUI_CONTEXT};
use crate::{DrawingContext, FuiDrawingContext, WindowOptions, APPLICATION_VM_CONTEXT};
use anyhow::Result;
use drawing::primitive::Primitive;
use drawing_gl::GlContextData;
use drawing_gl::GlRenderTarget;
use fui_core::{Children, Grid, Rect, Services, Size, ViewContext};
use fui_core::{ControlObject, EventProcessor, ObservableVec};
use fui_core::{ViewModel, WindowService};
use fui_macros::ui;
use std::cell::RefCell;
use std::ptr::null;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};
use typemap::TypeMap;
use windowing_api::{CursorShape, Edge};

pub type WindowId = i64;

///
/// Window data available only from the GUI thread.
///
pub struct WindowGUIThreadData {
    system_window: Option<windowing_qt::Window>,
    gl_context_data: Option<GlContextData>,
}

impl Drop for WindowGUIThreadData {
    fn drop(&mut self) {
        // It is important to drop window before drawing_context!
        // Window cleanups graphics resources and drawing context drops graphics device.
        self.system_window.take();
    }
}

///
/// Window data available only from the VM (View Models) thread.
///
pub struct WindowVMThreadData {
    id: WindowId,

    event_processor: RefCell<EventProcessor>,
    root_control: Rc<RefCell<dyn ControlObject>>,
    view: RefCell<Option<Rc<RefCell<dyn ControlObject>>>>,
    services: RefCell<Option<fui_core::Services>>,

    control_layers: ObservableVec<Rc<RefCell<dyn ControlObject>>>,
}

#[derive(Clone)]
pub struct Window {
    data: Rc<WindowVMThreadData>,
}

impl Window {
    pub async fn create(window_options: WindowOptions) -> Result<Self> {
        // VM Thread
        let (tx, rx) = oneshot::channel::<WindowId>();
        windowing_qt::Application::post_func(move || {
            // GUI Thread
            let mut native_window = windowing_qt::Window::new(None).unwrap();
            native_window.set_title(&window_options.title).unwrap();
            native_window
                .set_stay_on_top(window_options.stay_on_top)
                .unwrap();
            native_window
                .set_transparent_for_input(window_options.transparent_for_input)
                .unwrap();
            native_window
                .set_translucent_background(window_options.translucent_effect)
                .unwrap();
            native_window
                .set_frame_type(window_options.frame_type)
                .unwrap();
            native_window.resize(window_options.width, window_options.height);
            native_window.set_visible(window_options.visible).unwrap();
            if window_options.icon.len() > 0 {
                let icon = windowing_qt::Icon::from_data(&window_options.icon).unwrap();
                native_window.set_icon(&icon).unwrap();
            }

            let drawing_context = APPLICATION_GUI_CONTEXT.with(|context| {
                context
                    .borrow_mut()
                    .as_ref()
                    .unwrap()
                    .drawing_context
                    .clone()
            });

            let window_gui_thread_data = WindowGUIThreadData {
                system_window: Some(native_window),
                gl_context_data: None,
            };

            let window_id = APPLICATION_GUI_CONTEXT.with(move |context| {
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

            tx.send(window_id).unwrap();
        });

        let window_id = rx.await?;

        let control_layers = ObservableVec::<Rc<RefCell<dyn ControlObject>>>::new();

        let content = ui!(
            Grid {
                &control_layers,
            }
        );

        let window_data_rc = Rc::new(WindowVMThreadData {
            id: window_id,
            event_processor: RefCell::new(EventProcessor::new()),
            root_control: content,
            view: RefCell::new(None),
            services: RefCell::new(None),
            control_layers,
        });

        let window_service_rc: Rc<dyn WindowService> = window_data_rc.clone();
        let services = fui_core::Services::new(&window_service_rc, Rc::new(AppFileDialog {}));
        window_data_rc
            .root_control
            .borrow_mut()
            .get_context_mut()
            .set_services(Some(services.clone()));
        {
            let mut window_data_rc_services = window_data_rc.services.borrow_mut();
            *window_data_rc_services = Some(services);
        }

        let window = Window {
            data: window_data_rc,
        };

        APPLICATION_VM_CONTEXT.with({
            let window = window.clone();
            move |context| {
                context
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .windows
                    .insert(window_id, Rc::downgrade(&window.data))
            }
        });

        Ok(window)
    }

    pub fn downgrade(&self) -> WindowWeakAsync {
        WindowWeakAsync {
            data: Rc::downgrade(&self.data),
        }
    }

    pub fn get_id(&self) -> WindowId {
        self.data.id
    }

    pub fn set_vm<V: ViewModel>(&mut self, view_model: Rc<V>) {
        let new_view = ViewModel::create_view(&view_model);

        let mut window_data = self.data.clone();
        if let Some(view) = window_data.view.take() {
            window_data.remove_layer(&view);
        }
        window_data.add_layer(new_view.clone());
        window_data.view.borrow_mut().replace(new_view);
    }

    pub fn get_window_service(&self) -> Rc<dyn fui_core::WindowService + 'static> {
        let service: Rc<dyn fui_core::WindowService + 'static> = self.data.clone();
        service
    }

    pub fn get_services(&self) -> Services {
        self.data.services.borrow().clone().unwrap()
    }

    fn setup_window_events(window_id: WindowId, drawing_context: &Arc<Mutex<DrawingContext>>) {
        APPLICATION_GUI_CONTEXT.with(move |context| {
            let mut context = context.borrow_mut();
            let app_context = context.as_mut().unwrap();

            if let Some(window_data) = app_context.windows.get_mut(&window_id) {
                window_data.system_window.as_mut().unwrap().on_paint_gl({
                    let drawing_context_clone = drawing_context.clone();

                    move || {
                        let drawing_context_clone = drawing_context_clone.clone();
                        APPLICATION_GUI_CONTEXT.with(move |context| {
                            let mut context = context.borrow_mut();
                            let app_context = context.as_mut().unwrap();
                            if let Some(window_data) = app_context.windows.get_mut(&window_id) {
                                let background_texture_id;

                                {
                                    let mut drawing_context = drawing_context_clone.lock().unwrap();

                                    if window_data.gl_context_data.is_none() {
                                        window_data.gl_context_data =
                                            Some(drawing_context.device.init_context(|symbol| {
                                                window_data
                                                    .system_window
                                                    .as_ref()
                                                    .unwrap()
                                                    .get_opengl_proc_address(symbol)
                                                    .unwrap_or_else(|_| null())
                                            }));
                                    }

                                    background_texture_id =
                                        drawing_context.get_background_texture();
                                }

                                let width = window_data.system_window.as_mut().unwrap().get_width();
                                let height =
                                    window_data.system_window.as_mut().unwrap().get_height();
                                if width > 0 && height > 0 {
                                    Self::update_min_window_size(
                                        &app_context.func_gui2vm_thread_tx,
                                        window_id,
                                        window_data,
                                        &drawing_context_clone,
                                        background_texture_id,
                                    );

                                    Self::render(
                                        &app_context.func_gui2vm_thread_tx,
                                        window_id,
                                        window_data,
                                        &drawing_context_clone,
                                        width as u32,
                                        height as u32,
                                        background_texture_id,
                                    );
                                }
                            }
                        });
                    }
                });

                window_data.system_window.as_mut().unwrap().on_event({
                    let drawing_context_clone = drawing_context.clone();

                    move |event| {
                        let drawing_context_clone = drawing_context_clone.clone();
                        if let Some(input_event) = crate::event_converter::convert_event(&event) {
                            APPLICATION_GUI_CONTEXT.with(move |context| {
                                let mut context = context.borrow_mut();
                                let app_context = context.as_mut().unwrap();
                                if let Some(window_data) = app_context.windows.get_mut(&window_id) {
                                    let system_window = window_data.system_window.as_mut().unwrap();

                                    let width = system_window.get_width();
                                    let height = system_window.get_height();

                                    app_context
                                        .func_gui2vm_thread_tx
                                        .send({
                                            let drawing_context = drawing_context_clone.clone();

                                            Box::new(move || {
                                                // VM Thread
                                                let window_data =
                                                    APPLICATION_VM_CONTEXT.with(move |context| {
                                                        context
                                                            .borrow()
                                                            .as_ref()
                                                            .unwrap()
                                                            .windows
                                                            .get(&window_id)
                                                            .unwrap()
                                                            .upgrade()
                                                    });

                                                if let Some(window_data) = window_data {
                                                    let mut drawing_context =
                                                        drawing_context.lock().unwrap();

                                                    let mut fui_drawing_context =
                                                        FuiDrawingContext::new(
                                                            (width as u16, height as u16),
                                                            &mut drawing_context,
                                                            0,
                                                        );

                                                    // events go to the window's root control
                                                    let root_control =
                                                        window_data.root_control.clone();
                                                    window_data
                                                        .event_processor
                                                        .borrow_mut()
                                                        .handle_event(
                                                            &root_control,
                                                            &mut fui_drawing_context,
                                                            &input_event,
                                                        );
                                                }
                                            })
                                        })
                                        .unwrap_or_else(|e| panic!("Cannot send GUI event! {e}"));

                                    true
                                } else {
                                    false
                                }
                            })
                        } else {
                            false
                        }
                    }
                });
            }
        });
    }

    fn update_min_window_size(
        func_gui2vm_thread_tx: &mpsc::UnboundedSender<Box<dyn 'static + Send + FnOnce()>>,
        window_id: WindowId,
        window_data: &mut WindowGUIThreadData,
        drawing_context: &Arc<Mutex<DrawingContext>>,
        background_texture: i32,
    ) {
        // GUI Thread
        let (tx, rx) = std::sync::mpsc::channel::<Option<Rect>>();
        func_gui2vm_thread_tx
            .send({
                let drawing_context = drawing_context.clone();

                Box::new(move || {
                    // VM Thread
                    let window_data = APPLICATION_VM_CONTEXT.with(move |context| {
                        context
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .windows
                            .get(&window_id)
                            .unwrap()
                            .upgrade()
                    });

                    if let Some(window_data) = window_data {
                        let size = Size::new(0.0f32, 0.0f32);

                        let mut drawing_context = drawing_context.lock().unwrap();

                        let mut fui_drawing_context = FuiDrawingContext::new(
                            (size.width as u16, size.height as u16),
                            &mut drawing_context,
                            background_texture,
                        );

                        let min_size = {
                            window_data
                                .root_control
                                .borrow_mut()
                                .measure(&mut fui_drawing_context, size);
                            window_data.root_control.borrow_mut().get_rect()
                        };

                        tx.send(Some(min_size)).unwrap();
                    } else {
                        tx.send(None).unwrap();
                    }
                })
            })
            .unwrap_or_else(|e| panic!("Cannot update min window size! {e}"));

        let min_size = rx.recv().unwrap();

        if let Some(min_size) = min_size {
            window_data
                .system_window
                .as_mut()
                .unwrap()
                .set_minimum_size(min_size.width as i32, min_size.height as i32);
        }
    }

    fn render(
        func_gui2vm_thread_tx: &mpsc::UnboundedSender<Box<dyn 'static + Send + FnOnce()>>,
        window_id: WindowId,
        window_data: &mut WindowGUIThreadData,
        drawing_context: &Arc<Mutex<DrawingContext>>,
        width: u32,
        height: u32,
        background_texture: i32,
    ) {
        // GUI Thread
        let (tx, rx) = std::sync::mpsc::channel::<Option<Vec<Primitive>>>();
        func_gui2vm_thread_tx
            .send({
                let drawing_context = drawing_context.clone();

                Box::new(move || {
                    // VM Thread
                    let window_data = APPLICATION_VM_CONTEXT.with(move |context| {
                        context
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .windows
                            .get(&window_id)
                            .unwrap()
                            .upgrade()
                    });

                    if let Some(window_data) = window_data {
                        let size = Size::new(width as f32, height as f32);

                        let mut drawing_context = drawing_context.lock().unwrap();

                        let mut fui_drawing_context = FuiDrawingContext::new(
                            (size.width as u16, size.height as u16),
                            &mut drawing_context,
                            background_texture,
                        );

                        let mut primitives = Vec::new();

                        // background texture
                        primitives.push(drawing::primitive::Primitive::Image {
                            resource_key: background_texture,
                            rect: drawing::units::PixelRect::new(
                                drawing::units::PixelPoint::new(0.0f32, 0.0f32),
                                drawing::units::PixelSize::new(size.width, size.height),
                            ),
                            uv: [
                                0.0f32,
                                0.0f32,
                                1.0f32 * size.width / 256.0f32,
                                1.0f32 * size.height / 256.0f32,
                            ],
                        });

                        window_data
                            .root_control
                            .borrow_mut()
                            .measure(&mut fui_drawing_context, size);
                        window_data.root_control.borrow_mut().set_rect(
                            &mut fui_drawing_context,
                            Rect::new(0f32, 0f32, size.width, size.height),
                        );

                        let (mut primitives1, mut overlay) = window_data
                            .root_control
                            .borrow()
                            .to_primitives(&mut fui_drawing_context);
                        primitives.append(&mut primitives1);
                        primitives.append(&mut overlay);

                        window_data
                            .root_control
                            .borrow_mut()
                            .get_context_mut()
                            .set_is_dirty(false);

                        tx.send(Some(primitives)).unwrap();
                    } else {
                        tx.send(None).unwrap();
                    }
                }) as Box<dyn 'static + Send + FnOnce()>
            })
            .unwrap_or_else(|e| panic!("Cannot render! {e}"));

        let primitives = rx.recv().unwrap();

        if let Some(primitives) = primitives {
            let mut drawing_context = drawing_context.lock().unwrap();
            let res = drawing_context.begin(window_data.gl_context_data.as_ref().unwrap());
            if let Err(err) = res {
                eprintln!("Render error on begin drawing: {}", err);
            } else {
                let render_target = GlRenderTarget::new(0, width as u16, height as u16, 1.0f32);

                drawing_context.clear(&render_target, &[0.3f32, 0.4f32, 0.3f32, 1.0f32]);
                let res = drawing_context.draw(&render_target, &primitives);
                if let Err(err) = res {
                    eprintln!("Render error: {}", err);
                }
                drawing_context.end(window_data.gl_context_data.as_ref().unwrap());
            }
        }
    }
}

impl Drop for WindowVMThreadData {
    fn drop(&mut self) {
        let window_id = self.id;
        windowing_qt::Application::post_func(move || {
            APPLICATION_GUI_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let app_context = context.as_mut().unwrap();
                app_context.windows.remove(&window_id);
            });
        });
    }
}

impl fui_core::WindowService for WindowVMThreadData {
    fn add_layer(&self, control: Rc<RefCell<dyn ControlObject>>) {
        self.control_layers.push(control);
    }

    fn remove_layer(&self, control: &Rc<RefCell<dyn ControlObject>>) {
        self.control_layers.retain(|el| !Rc::ptr_eq(el, control));
    }

    fn repaint(&self) {
        let window_id = self.id;
        windowing_qt::Application::post_func(move || {
            APPLICATION_GUI_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let app_context = context.as_mut().unwrap();
                if let Some(window) = app_context.windows.get_mut(&window_id) {
                    window.system_window.as_mut().unwrap().update();
                }
            });
        });
    }

    fn set_cursor(&self, cursor_shape: CursorShape) {
        let window_id = self.id;
        windowing_qt::Application::post_func(move || {
            APPLICATION_GUI_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let app_context = context.as_mut().unwrap();
                if let Some(window) = app_context.windows.get_mut(&window_id) {
                    window
                        .system_window
                        .as_mut()
                        .unwrap()
                        .set_cursor(cursor_shape);
                }
            });
        });
    }

    fn start_system_move(&self) {
        let window_id = self.id;
        windowing_qt::Application::post_func(move || {
            APPLICATION_GUI_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let app_context = context.as_mut().unwrap();
                if let Some(window) = app_context.windows.get_mut(&window_id) {
                    window.system_window.as_mut().unwrap().start_system_move();
                }
            });
        });
    }

    fn start_system_resize(&self, edges: Edge) {
        let window_id = self.id;
        windowing_qt::Application::post_func(move || {
            APPLICATION_GUI_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let app_context = context.as_mut().unwrap();
                if let Some(window) = app_context.windows.get_mut(&window_id) {
                    window
                        .system_window
                        .as_mut()
                        .unwrap()
                        .start_system_resize(edges);
                }
            });
        });
    }
}

pub struct WindowWeakAsync {
    data: Weak<WindowVMThreadData>,
}

impl WindowWeakAsync {
    pub fn upgrade(&self) -> Option<Window> {
        self.data.upgrade().map(|d| Window { data: d })
    }
}
