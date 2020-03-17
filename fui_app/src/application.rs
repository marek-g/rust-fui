use fui::*;

use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use winit::dpi::LogicalSize;

use crate::Dispatcher;
use crate::DrawingContext;
use crate::DrawingWindowTarget;
use crate::Window;
use crate::WindowManager;

pub struct Application {
    title: &'static str,
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    event_loop_iteration: Rc<RefCell<Event<()>>>,
    drawing_context: Rc<RefCell<DrawingContext>>,
    window_manager: Rc<RefCell<WindowManager>>,
}

impl Application {
    pub fn new(title: &'static str) -> Result<Self> {
        crate::high_dpi::set_process_high_dpi_aware();

        let event_loop = winit::event_loop::EventLoop::new();

        let drawing_context = Rc::new(RefCell::new(DrawingContext::new()?));

        Dispatcher::setup_events_loop_proxy(event_loop.create_proxy());

        Ok(Application {
            title: title,
            event_loop: Some(event_loop),
            event_loop_iteration: Rc::new(RefCell::new(Event::new())),
            drawing_context: drawing_context.clone(),
            window_manager: Rc::new(RefCell::new(WindowManager::new(drawing_context))),
        })
    }

    pub fn get_title(&self) -> &'static str {
        self.title
    }

    pub fn get_event_loop(&self) -> Option<&winit::event_loop::EventLoop<()>> {
        self.event_loop.as_ref()
    }

    pub fn get_drawing_context(&self) -> &Rc<RefCell<DrawingContext>> {
        &self.drawing_context
    }

    pub fn get_window_manager(&self) -> &Rc<RefCell<WindowManager>> {
        &self.window_manager
    }

    pub fn create_loop_proxy(&self) -> Option<winit::event_loop::EventLoopProxy<()>> {
        self.event_loop.as_ref().map(|el| el.create_proxy())
    }

    pub fn get_event_loop_interation(&self) -> &Rc<RefCell<Event<()>>> {
        &self.event_loop_iteration
    }

    pub fn add_window<V: RcView>(
        &mut self,
        window_builder: winit::window::WindowBuilder,
        view_model: Rc<RefCell<V>>,
    ) -> Result<winit::window::WindowId> {
        self.window_manager.borrow_mut().add_window_view_model(
            window_builder,
            self.get_event_loop().unwrap(),
            &view_model,
        )
    }

    pub fn run(&mut self) {
        let mut frame_no = 0;

        let event_loop = self.event_loop.take().unwrap();
        let event_loop_iteration = self.event_loop_iteration.clone();
        let drawing_context = self.drawing_context.clone();
        let window_manager = self.window_manager.clone();

        event_loop.run(move |event, _, control_flow| {
            event_loop_iteration.borrow_mut().emit(());
            CallbackExecutor::execute_all_in_queue();
            Dispatcher::execute_all_in_queue();

            match event {
                winit::event::Event::EventsCleared => {
                    for mut window in window_manager.borrow_mut().get_windows_mut().values_mut() {
                        if Application::is_dirty(&mut window) {
                            window.drawing_window_target.get_window().request_redraw();
                        }
                    }
                }

                winit::event::Event::WindowEvent {
                    ref window_id,
                    ref event,
                } => {
                    if let Some(window) = window_manager
                        .borrow_mut()
                        .get_windows_mut()
                        .get_mut(window_id)
                    {
                        match event {
                            winit::event::WindowEvent::CloseRequested => {
                                *control_flow = winit::event_loop::ControlFlow::Exit;
                            }

                            winit::event::WindowEvent::RedrawRequested => {
                                let logical_size =
                                    window.drawing_window_target.get_window().inner_size();
                                let physical_size = logical_size.to_physical(
                                    window.drawing_window_target.get_window().hidpi_factor(),
                                );
                                if physical_size.width > 0.0 && physical_size.height > 0.0 {
                                    let cpu_time = cpu_time::ProcessTime::now();

                                    Application::render(
                                        window,
                                        &mut drawing_context.borrow_mut(),
                                        physical_size.width as u32,
                                        physical_size.height as u32,
                                    );

                                    let cpu_time = cpu_time.elapsed();
                                    frame_no += 1;
                                    println!("Frame no: {}, CPU time: {:?}", frame_no, cpu_time);

                                    window.drawing_window_target.swap_buffers();
                                }
                            }

                            winit::event::WindowEvent::Resized(logical_size) => {
                                let physical_size = logical_size.to_physical(
                                    window.drawing_window_target.get_window().hidpi_factor(),
                                );

                                let drawing_context = &mut drawing_context.borrow_mut();
                                drawing_context.update_size(
                                    &mut window.drawing_window_target,
                                    physical_size.width as u16,
                                    physical_size.height as u16,
                                );

                                if let Some(ref mut root_view) = window.root_view {
                                    let size = Size::new(
                                        physical_size.width as f32,
                                        physical_size.height as f32,
                                    );
                                    let mut root_control = root_view.borrow_mut();
                                    root_control.get_context_mut().set_is_dirty(true);
                                    root_control.measure(drawing_context.deref_mut(), size);
                                    root_control.set_rect(Rect::new(
                                        0f32,
                                        0f32,
                                        size.width,
                                        size.height,
                                    ));
                                }
                            }

                            _ => (),
                        }

                        if let Some(ref mut root_view) = window.root_view {
                            if let Some(input_event) = crate::event_converter::convert_event(event)
                            {
                                window.event_processor.handle_event(
                                    root_view,
                                    drawing_context.borrow_mut().deref_mut(),
                                    &input_event,
                                );
                            }
                        }
                    }
                }

                _ => {
                    // As of winit = "0.20.0-alpha4".
                    //
                    // The event loop should be in Wait mode to not waste CPU cycles,
                    // however currently it's not working with the LMB pressed event,
                    // the pressed event is received late - along with the LMB release event.
                    //
                    // As a workaround we are using WaitUntil here but it will be better
                    // to go back to Wait mode when the bug in winit is fixed.
                    //
                    // What is probably happening is - winit is generating events:
                    // - DeviceEvent LMB pressed
                    // - EventsCleared
                    // - NewEvents / WaitCancelled
                    // - WindowEvent / MouseInput LMB pressed
                    // Calling request_redraw() in EventsCleared puts the RedrawRequested
                    // event before NewEvents
                    //
                    // More info: https://github.com/rust-windowing/winit/issues/1041
                    //
                    // *control_flow = winit::event_loop::ControlFlow::Wait,
                    *control_flow = winit::event_loop::ControlFlow::WaitUntil(
                        std::time::Instant::now() + std::time::Duration::from_millis(10),
                    );
                }
            };
        });
    }

    fn is_dirty(window: &mut Window<DrawingWindowTarget>) -> bool {
        if let Some(ref mut root_view) = window.root_view {
            let root_control = root_view.borrow();
            if root_control.get_context().is_dirty() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn render(
        window: &mut Window<DrawingWindowTarget>,
        drawing_context: &mut DrawingContext,
        width: u32,
        height: u32,
    ) {
        if let Some(ref mut root_view) = window.root_view {
            let mut root_control = root_view.borrow_mut();

            let size = Size::new(width as f32, height as f32);
            root_control.measure(drawing_context, size);
            root_control.set_rect(Rect::new(0f32, 0f32, size.width, size.height));

            let primitives = root_control.to_primitives(drawing_context);

            let res = drawing_context.begin(&mut window.drawing_window_target);
            if let Err(err) = res {
                eprintln!("Render error on begin drawing: {}", err);
            } else {
                drawing_context.clear(
                    window.drawing_window_target.get_render_target(),
                    &[0.3f32, 0.4f32, 0.3f32, 1.0f32],
                );
                let res = drawing_context.draw(
                    window.drawing_window_target.get_render_target(),
                    &primitives,
                );
                if let Err(err) = res {
                    eprintln!("Render error: {}", err);
                }
                drawing_context.end(&mut window.drawing_window_target);
            }

            root_control.get_context_mut().set_is_dirty(false);
        }
    }
}
