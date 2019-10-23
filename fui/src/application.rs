extern crate winit;

use Result;

use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::LogicalSize;

use common::*;
use drawing::backend::WindowTarget;
use drawing::units::PhysPixelSize;
use drawing_context::DrawingContext;
use events::*;
use observable::Event;
use CallbackExecutor;
use Dispatcher;
use Window;
use WindowManager;

pub struct Application {
    title: &'static str,
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    event_loop_iteration: Rc<RefCell<Event<()>>>,
    drawing_context: Rc<RefCell<DrawingContext>>,
    window_manager: Rc<RefCell<WindowManager>>,
}

impl Application {
    pub fn new(title: &'static str) -> Result<Self> {
        ::high_dpi::set_process_high_dpi_aware();

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

    pub fn run(&mut self) {
        let mut frame_no = 0;

        let event_loop = self.event_loop.take().unwrap();
        let event_loop_iteration = self.event_loop_iteration.clone();
        let mut event_processor = EventProcessor::new();
        let drawing_context = self.drawing_context.clone();
        let window_manager = self.window_manager.clone();

        event_loop.run(move |event, _, control_flow| {
            match event {
                winit::event::Event::EventsCleared => {
                    event_loop_iteration.borrow_mut().emit(());
                    CallbackExecutor::execute_all_in_queue();
                    Dispatcher::execute_all_in_queue();

                    // if dirty queue a RedrawRequested event
                    for window in window_manager.borrow_mut().get_windows_mut().values_mut() {
                        let logical_size = window.get_drawing_target().get_window().inner_size();
                        let physical_size = logical_size
                            .to_physical(window.get_drawing_target().get_window().hidpi_factor());
                        if physical_size.width > 0.0 && physical_size.height > 0.0 {
                            if let Some(ref mut root_view) = window.get_root_view_mut() {
                                let root_control = root_view.borrow();
                                if root_control.is_dirty() {
                                    frame_no += 1;
                                    println!("Frame no: {}", frame_no);
                                }
                            }

                            let need_swap_buffers = Application::render(
                                window,
                                &mut drawing_context.borrow_mut(),
                                physical_size.width as u32,
                                physical_size.height as u32,
                            );
                            window.set_need_swap_buffers(need_swap_buffers);
                            window.get_drawing_target().get_window().request_redraw();
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
                                if window.get_need_swap_buffers() {
                                    println!("-- Swap buffers");
                                    window.get_drawing_target_mut().swap_buffers();
                                }
                            }

                            winit::event::WindowEvent::Resized(logical_size) => {
                                let physical_size = logical_size.to_physical(
                                    window.get_drawing_target().get_window().hidpi_factor(),
                                );

                                let drawing_context = &mut drawing_context.borrow_mut();
                                drawing_context.update_size(
                                    window.get_drawing_target_mut(),
                                    physical_size.width as u16,
                                    physical_size.height as u16,
                                );

                                if let Some(ref mut root_view) = window.get_root_view_mut() {
                                    let size = Size::new(
                                        physical_size.width as f32,
                                        physical_size.height as f32,
                                    );
                                    let mut root_control = root_view.borrow_mut();
                                    root_control.set_is_dirty(true);
                                    root_control.measure(drawing_context, size);
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

                        event_processor.handle_event(window, event);
                    }
                }

                _ => {}
            };

            event_loop_iteration.borrow_mut().emit(());
            CallbackExecutor::execute_all_in_queue();
            Dispatcher::execute_all_in_queue();
        });
    }

    fn render(
        window: &mut Window,
        drawing_context: &mut DrawingContext,
        width: u32,
        height: u32,
    ) -> bool {
        let (drawing_target, root_view) = window.get_drawing_target_and_root_view_mut();
        if let Some(ref mut root_view) = root_view {
            let mut root_control = root_view.borrow_mut();

            if root_control.is_dirty() {
                let size = Size::new(width as f32, height as f32);
                root_control.measure(drawing_context, size);
                root_control.set_rect(Rect::new(0f32, 0f32, size.width, size.height));

                let primitives = root_control.to_primitives(drawing_context);

                let res = drawing_context.begin(drawing_target);
                if let Err(err) = res {
                    eprintln!("Render error on begin drawing: {}", err);
                } else {
                    drawing_context.clear(
                        drawing_target.get_render_target(),
                        &[0.5f32, 0.4f32, 0.3f32, 1.0f32],
                    );
                    let res = drawing_context.draw(
                        drawing_target.get_render_target(),
                        PhysPixelSize::new(width as f32, height as f32),
                        primitives,
                    );
                    if let Err(err) = res {
                        eprintln!("Render error: {}", err);
                    }
                    drawing_context.end(drawing_target);
                }

                root_control.set_is_dirty(false);

                return true;
            }
        }

        false
    }
}
