use crate::{DrawingContext, FuiDrawingContext, WindowOptions};
use anyhow::Result;
use drawing_gl::{GlContextData, GlRenderTarget};
use fui_core::Rect;
use fui_core::Size;
use fui_core::ViewModel;
use fui_core::WindowService;
use fui_core::{Children, Grid, ViewContext};
use fui_core::{ControlObject, EventProcessor, ObservableVec};
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use typemap::TypeMap;

struct WindowData {
    system_window: fui_system::Window,
    gl_context_data: Option<GlContextData>,

    event_processor: EventProcessor,
    root_control: Rc<RefCell<dyn ControlObject>>,
    view: Option<Rc<RefCell<dyn ControlObject>>>,
    services: Option<Rc<RefCell<fui_core::Services>>>,

    control_layers: ObservableVec<Rc<RefCell<dyn ControlObject>>>,
}

#[derive(Clone)]
pub struct Window {
    data: Rc<RefCell<WindowData>>,
}

impl Window {
    pub fn create(
        window_options: WindowOptions,
        drawing_context: &Rc<RefCell<DrawingContext>>,
    ) -> Result<Self> {
        let mut native_window = fui_system::Window::new(None)?;
        native_window.set_title(&window_options.title)?;
        native_window.resize(window_options.width, window_options.height);
        native_window.set_visible(window_options.visible)?;
        if window_options.icon.len() > 0 {
            let icon = fui_system::Icon::from_data(&window_options.icon)?;
            native_window.set_icon(&icon)?;
        }

        let control_layers = ObservableVec::<Rc<RefCell<dyn ControlObject>>>::new();

        let content = ui!(
            Grid {
                &control_layers,
            }
        );

        let window_data_rc = Rc::new(RefCell::new(WindowData {
            system_window: native_window,
            gl_context_data: None,
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

        let window = Window {
            data: window_data_rc,
        };

        window.setup_window_events(&drawing_context);

        Ok(window)
    }

    pub fn downgrade(&self) -> WindowWeak {
        WindowWeak {
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

    pub fn setup_window_events(&self, drawing_context: &Rc<RefCell<DrawingContext>>) {
        let window = &mut self.data.borrow_mut().system_window;

        window.on_paint_gl({
            let window_weak = self.downgrade();
            let drawing_context_clone = drawing_context.clone();
            let mut initialized = false;

            move || {
                if let Some(window) = window_weak.upgrade() {
                    let window_data = &mut window.data.borrow_mut();
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
                    if width > 0 && height > 0 {
                        Self::update_min_window_size(window_data, &mut drawing_context, 0);

                        Self::render(
                            window_data,
                            &mut drawing_context,
                            width as u32,
                            height as u32,
                            0,
                        );
                    }
                }
            }
        });

        window.on_event({
            let window_weak = self.downgrade();
            let drawing_context_clone = drawing_context.clone();

            move |event| {
                if let Some(window) = window_weak.upgrade() {
                    if let Some(input_event) = crate::event_converter::convert_event(&event) {
                        let window_data = &mut window.data.borrow_mut();
                        let system_window = &mut window_data.system_window;
                        let mut drawing_context = drawing_context_clone.borrow_mut();

                        let width = system_window.get_width();
                        let height = system_window.get_height();
                        let mut fui_drawing_context = FuiDrawingContext::new(
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
                        );

                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        });
    }

    fn update_min_window_size(
        window_data: &mut WindowData,
        drawing_context: &mut DrawingContext,
        background_texture: i32,
    ) {
        let size = Size::new(0.0f32, 0.0f32);

        let mut fui_drawing_context = FuiDrawingContext::new(
            (size.width as u16, size.height as u16),
            drawing_context,
            background_texture,
        );

        let min_size = {
            window_data
                .root_control
                .borrow_mut()
                .measure(&mut fui_drawing_context, size);
            window_data.root_control.borrow_mut().get_rect()
        };

        window_data
            .system_window
            .set_minimum_size(min_size.width as i32, min_size.height as i32);
    }

    fn render(
        window_data: &mut WindowData,
        drawing_context: &mut DrawingContext,
        width: u32,
        height: u32,
        background_texture: i32,
    ) {
        let size = Size::new(width as f32, height as f32);

        let mut fui_drawing_context = FuiDrawingContext::new(
            (size.width as u16, size.height as u16),
            drawing_context,
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

impl fui_core::WindowService for WindowData {
    fn add_layer(&mut self, control: Rc<RefCell<dyn ControlObject>>) {
        self.control_layers.push(control);
    }

    fn remove_layer(&mut self, control: &Rc<RefCell<dyn ControlObject>>) {
        self.control_layers
            .remove_filter(|el| Rc::ptr_eq(el, control));
    }

    fn repaint(&mut self) {
        self.system_window.update();
    }
}

pub struct WindowWeak {
    data: Weak<RefCell<WindowData>>,
}

impl WindowWeak {
    pub fn upgrade(&self) -> Option<Window> {
        self.data.upgrade().map(|d| Window { data: d })
    }
}
