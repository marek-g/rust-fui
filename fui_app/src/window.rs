use crate::{drawing_context, AppFileDialog, APPLICATION_GUI_CONTEXT};
use crate::{WindowOptions, APPLICATION_VM_CONTEXT};
use anyhow::Result;
use fui_core::{Children, FuiDrawingContext, Grid, Rect, Services, Size, ViewContext};
use fui_core::{ControlObject, EventProcessor, ObservableVec};
use fui_core::{ViewModel, WindowService};
use fui_drawing::prelude::*;
use fui_macros::ui;
use std::cell::RefCell;
use std::ptr::null_mut;
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

            let window_gui_thread_data = WindowGUIThreadData {
                system_window: Some(native_window),
            };

            let window_id = APPLICATION_GUI_CONTEXT.with(move |context| {
                let mut context = context.borrow_mut();
                let app_context = context.as_mut().unwrap();

                let window_id = app_context.next_window_id;
                app_context
                    .windows
                    .insert(window_id, window_gui_thread_data);

                app_context.next_window_id += 1;

                window_id
            });

            Self::setup_window_events(window_id);

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

        let window_data = self.data.clone();
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

    fn setup_window_events(window_id: WindowId) {
        APPLICATION_GUI_CONTEXT.with(move |context| {
            let mut context = context.borrow_mut();
            let app_context = context.as_mut().unwrap();

            if let Some(window_data) = app_context.windows.get_mut(&window_id) {
                window_data.system_window.as_mut().unwrap().on_paint_gl({
                    move || {
                        APPLICATION_GUI_CONTEXT.with(move |context| {
                            let mut context = context.borrow_mut();
                            let app_context = context.as_mut().unwrap();

                            // create application wide OpenGl context if not done before
                            if let None = app_context.drawing_context_gl {
                                if let Some(window_data) = app_context.windows.get_mut(&window_id) {
                                    let drawing_context_gl = unsafe {
                                        DrawingContextGl::new_gl(|symbol| {
                                            window_data
                                                .system_window
                                                .as_ref()
                                                .unwrap()
                                                .get_opengl_proc_address(symbol)
                                                .unwrap_or_else(|_| null_mut())
                                        })
                                        .unwrap()
                                    };
                                    app_context.drawing_context_gl =
                                        Some(Arc::new(Mutex::new(drawing_context_gl)));
                                }
                            }

                            let drawing_context =
                                app_context.drawing_context_gl.as_ref().unwrap().clone();

                            if let Some(window_data) = app_context.windows.get_mut(&window_id) {
                                let background_texture_id = -1;

                                /*{
                                    let mut drawing_context = drawing_context_clone.lock().unwrap();

                                    if window_data.gl_context_data.is_none() {
                                        window_data.gl_context_data =
                                            Some(drawing_context.device.init_context(|symbol| {
                                                window_data
                                                    .system_window
                                                    .as_ref()
                                                    .unwrap()
                                                    .get_opengl_proc_address(symbol)
                                                    .unwrap_or_else(|_| null_mut())
                                            }));
                                    }

                                    background_texture_id =
                                        drawing_context.get_background_texture();
                                }*/

                                let width = window_data.system_window.as_mut().unwrap().get_width();
                                let height =
                                    window_data.system_window.as_mut().unwrap().get_height();
                                if width > 0 && height > 0 {
                                    Self::update_min_window_size(
                                        &app_context.func_gui2vm_thread_tx,
                                        window_id,
                                        window_data,
                                        background_texture_id,
                                    );

                                    Self::render(
                                        &app_context.func_gui2vm_thread_tx,
                                        &drawing_context,
                                        window_id,
                                        window_data,
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
                    move |event| {
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
                                                    let mut fonts = DrawingFonts::default();
                                                    let mut display_list_builder =
                                                        DrawingDisplayListBuilder::new(None);
                                                    let mut overlay_list_builder =
                                                        DrawingDisplayListBuilder::new(None);

                                                    let mut fui_drawing_context =
                                                        FuiDrawingContext {
                                                            fonts: &mut fonts,
                                                            display: &mut display_list_builder,
                                                            overlay: &mut overlay_list_builder,
                                                        };

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
        background_texture: i32,
    ) {
        // GUI Thread
        let (tx, rx) = std::sync::mpsc::channel::<Option<Rect>>();
        func_gui2vm_thread_tx
            .send({
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

                        let mut fonts = DrawingFonts::default();
                        let mut display_list_builder = DrawingDisplayListBuilder::new(None);
                        let mut overlay_list_builder = DrawingDisplayListBuilder::new(None);

                        let mut fui_drawing_context = FuiDrawingContext {
                            fonts: &mut fonts,
                            display: &mut display_list_builder,
                            overlay: &mut overlay_list_builder,
                        };

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
        drawing_context: &Arc<Mutex<DrawingContextGl>>,
        window_id: WindowId,
        window_data: &mut WindowGUIThreadData,
        width: u32,
        height: u32,
        background_texture: i32,
    ) {
        // GUI Thread
        let (tx, rx) = std::sync::mpsc::channel::<Option<fui_drawing::DrawingDisplayListBuilder>>();
        func_gui2vm_thread_tx
            .send({
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

                        let mut fonts = DrawingFonts::default();
                        let mut display_list_builder = DrawingDisplayListBuilder::new(Some(rect(
                            0.0,
                            0.0,
                            size.width,
                            size.height,
                        )));
                        display_list_builder.draw_paint([0.3f32, 0.4f32, 0.3f32, 1.0f32]);
                        let mut overlay_list_builder = DrawingDisplayListBuilder::new(Some(rect(
                            0.0,
                            0.0,
                            size.width,
                            size.height,
                        )));

                        let mut fui_drawing_context = FuiDrawingContext {
                            fonts: &mut fonts,
                            display: &mut display_list_builder,
                            overlay: &mut overlay_list_builder,
                        };

                        /*let mut primitives = Vec::new();

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
                        });*/

                        window_data
                            .root_control
                            .borrow_mut()
                            .measure(&mut fui_drawing_context, size);
                        window_data.root_control.borrow_mut().set_rect(
                            &mut fui_drawing_context,
                            Rect::new(0f32, 0f32, size.width, size.height),
                        );

                        window_data
                            .root_control
                            .borrow_mut()
                            .draw(&mut fui_drawing_context);

                        window_data
                            .root_control
                            .borrow_mut()
                            .get_context_mut()
                            .set_is_dirty(false);

                        tx.send(Some(display_list_builder)).unwrap();
                    } else {
                        tx.send(None).unwrap();
                    }
                }) as Box<dyn 'static + Send + FnOnce()>
            })
            .unwrap_or_else(|e| panic!("Cannot render! {e}"));

        let display_list_builder = rx.recv().unwrap();

        if let Some(display_list_builder) = display_list_builder {
            let display_list = display_list_builder.build().unwrap();

            let framebuffer_id = window_data
                .system_window
                .as_ref()
                .unwrap()
                .get_default_framebuffer_id();

            let mut drawing_surface = unsafe {
                drawing_context
                    .lock()
                    .unwrap()
                    .wrap_gl_framebuffer(
                        framebuffer_id,
                        width as u32,
                        height as u32,
                        fui_drawing::ColorFormat::RGBA,
                    )
                    .unwrap()
            };

            let res = drawing_surface.draw(&display_list);
            if let Err(err) = res {
                eprintln!("Render error: {}", err);
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
