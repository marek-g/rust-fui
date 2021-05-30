use fui_core::*;

use anyhow::Result;

use std::cell::RefCell;
use std::rc::Rc;

use crate::DrawingContext;
use crate::FuiDrawingContext;
use crate::GlWindow;
use drawing_gl::GlRenderTarget;
use rand::{thread_rng, Rng};

pub struct Application {
    app: fui_system::Application,
    title: &'static str,
    event_loop_iteration: Rc<RefCell<Event<()>>>,
    drawing_context: Rc<RefCell<DrawingContext>>,

    windows: Vec<Rc<RefCell<Window<GlWindow>>>>,
    window_services: Vec<Rc<RefCell<Services>>>,
}

impl Application {
    pub fn new(title: &'static str) -> Result<Self> {
        let app = fui_system::Application::new(
            fui_system::ApplicationOptionsBuilder::new()
                .with_title(title)
                .with_opengl_share_contexts(true)
                .with_opengl_stencil_bits(8)
                .build(),
        )?;

        register_current_thread_dispatcher(Box::new(crate::dispatcher::Dispatcher(
            app.get_dispatcher(),
        )));

        let drawing_context = Rc::new(RefCell::new(DrawingContext::new()?));

        Ok(Application {
            app,
            title,
            event_loop_iteration: Rc::new(RefCell::new(Event::new())),
            drawing_context: drawing_context.clone(),
            windows: Vec::new(),
            window_services: Vec::new(),
        })
    }

    pub fn get_title(&self) -> &'static str {
        self.title
    }

    pub fn get_drawing_context(&self) -> &Rc<RefCell<DrawingContext>> {
        &self.drawing_context
    }

    pub fn get_event_loop_interation(&self) -> &Rc<RefCell<Event<()>>> {
        &self.event_loop_iteration
    }

    pub fn add_window<V: ViewModel>(
        &mut self,
        window: fui_system::Window,
        view_model: Rc<RefCell<V>>,
    ) -> Result<Rc<RefCell<Window<GlWindow>>>> {
        let mut window_rc = self.create_window(window)?;
        Self::set_window_vm(&window_rc, view_model);

        window_rc
            .borrow_mut()
            .native_window
            .window
            .set_visible(true);

        Ok(window_rc)
    }

    pub fn create_window(
        &mut self,
        native_window: fui_system::Window,
    ) -> Result<Rc<RefCell<Window<GlWindow>>>> {
        let window = Window::new(GlWindow::new(native_window));
        let window_rc = Rc::new(RefCell::new(window));

        let window_service_rc: Rc<RefCell<dyn WindowService>> = window_rc.clone();

        let services = Rc::new(RefCell::new(Services::new(&window_service_rc)));
        window_rc
            .borrow()
            .get_root_control()
            .borrow_mut()
            .get_context_mut()
            .set_services(Some(Rc::downgrade(&services)));

        self.setup_window(&window_rc);

        self.windows.push(window_rc.clone());
        self.window_services.push(services);

        Ok(window_rc)
    }

    pub fn set_window_vm<V: ViewModel>(
        window_rc: &Rc<RefCell<Window<GlWindow>>>,
        view_model: Rc<RefCell<V>>,
    ) {
        let view = ViewModel::create_view(&view_model);
        window_rc.borrow_mut().add_layer(view);
    }

    pub fn run(&mut self) {
        self.app.message_loop();
    }

    pub fn exit() {
        fui_system::Application::exit(0);
    }

    fn is_dirty(window: &mut Window<GlWindow>) -> bool {
        window.get_root_control().borrow().get_context().is_dirty()
    }

    fn update_min_window_size(
        window: &mut Window<GlWindow>,
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
            let mut root_control = window.get_root_control().borrow_mut();
            root_control.measure(&mut fui_drawing_context, size);
            root_control.get_rect()
        };

        window
            .native_window
            .window
            .set_minimum_size(min_size.width as i32, min_size.height as i32);
    }

    fn render(
        window: &mut Window<GlWindow>,
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

        {
            let mut root_control = window.get_root_control().borrow_mut();

            root_control.measure(&mut fui_drawing_context, size);
            root_control.set_rect(
                &mut fui_drawing_context,
                Rect::new(0f32, 0f32, size.width, size.height),
            );

            let (mut primitives1, mut overlay) =
                root_control.to_primitives(&mut fui_drawing_context);
            primitives.append(&mut primitives1);
            primitives.append(&mut overlay);

            root_control.get_context_mut().set_is_dirty(false);
        }

        let res = drawing_context.begin(&mut window.native_window);
        if let Err(err) = res {
            eprintln!("Render error on begin drawing: {}", err);
        } else {
            let render_target = GlRenderTarget::new(0, width as u16, height as u16, 1.0f32);

            drawing_context.clear(&render_target, &[0.3f32, 0.4f32, 0.3f32, 1.0f32]);
            let res = drawing_context.draw(&render_target, &primitives);
            if let Err(err) = res {
                eprintln!("Render error: {}", err);
            }
            drawing_context.end(&mut window.native_window);
        }
    }

    fn setup_window(&self, window_rc: &Rc<RefCell<Window<GlWindow>>>) {
        let window = &mut window_rc.borrow_mut().native_window.window;

        window.on_paint_gl({
            let window_weak = Rc::downgrade(&window_rc);
            let drawing_context_clone = self.drawing_context.clone();
            let mut initialized = false;

            move || {
                if let Some(mut window) = window_weak.upgrade() {
                    let mut window = window.borrow_mut();
                    let mut drawing_context = drawing_context_clone.borrow_mut();

                    if !initialized {
                        window.native_window.gl_context_data =
                            Some(drawing_context.device.init_context(|symbol| {
                                window
                                    .native_window
                                    .window
                                    .get_opengl_proc_address(symbol)
                                    .unwrap()
                            }));
                        initialized = true;
                    }

                    let width = window.native_window.window.get_width();
                    let height = window.native_window.window.get_height();
                    if width > 0 && height > 0 {
                        Application::update_min_window_size(&mut window, &mut drawing_context, 0);

                        Application::render(
                            &mut window,
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
            let window_weak = Rc::downgrade(&window_rc);
            let drawing_context_clone = self.drawing_context.clone();

            move |event| {
                if let Some(mut window) = window_weak.upgrade() {
                    if let Some(input_event) = crate::event_converter::convert_event(&event) {
                        let mut window = window.borrow_mut();
                        let mut drawing_context = drawing_context_clone.borrow_mut();

                        let width = window.native_window.window.get_width();
                        let height = window.native_window.window.get_height();
                        let mut fui_drawing_context = FuiDrawingContext::new(
                            (width as u16, height as u16),
                            &mut drawing_context,
                            0,
                        );

                        // events go to the window's root control
                        let root_view = window.get_root_control().clone();
                        window.event_processor.handle_event(
                            &root_view,
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
}

impl Drop for Application {
    fn drop(&mut self) {
        // It is important to drop windows before drawing_context!
        // Windows cleanup graphics resources and drawing context drops graphics device.
        self.windows.clear();
        self.window_services.clear();
    }
}
