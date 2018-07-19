extern crate winit;

use ::Result;

use winit::dpi::LogicalSize;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use drawing::units::{ PhysPixelSize };
use drawing::backend::WindowTarget;
use drawing_context::DrawingContext;
use common::*;
use events::*;
use observable::Event;
use View;
use ViewData;
use CallbackExecutor;
use Dispatcher;
use Window;

pub struct Application {
    title: &'static str,
    events_loop: winit::EventsLoop,
    events_loop_interation: Event<()>,
    event_processor: EventProcessor,
    drawing_context: Rc<RefCell<DrawingContext>>,
    windows: HashMap<winit::WindowId, Window>,
}

impl Application {
    pub fn new(title: &'static str) -> Result<Self> {
        ::high_dpi::set_process_high_dpi_aware();

        let events_loop = winit::EventsLoop::new();

        let drawing_context = Rc::new(RefCell::new(DrawingContext::new()?));

        Dispatcher::setup_events_loop_proxy(events_loop.create_proxy());

        Ok(Application {
            title: title,
            events_loop: events_loop,
            events_loop_interation: Event::new(),
            event_processor: EventProcessor::new(),
            drawing_context: drawing_context,
            windows: HashMap::new(),
        })
    }

    pub fn add_window(&mut self, window_builder: winit::WindowBuilder, view_data: ViewData) -> Result<winit::WindowId>
    {
        let mut window_target = self.drawing_context.borrow_mut().create_window(window_builder, &self.events_loop)?;
        let logical_size = window_target.get_window().get_inner_size().unwrap_or(LogicalSize::new(0.0, 0.0));
        let window_id = window_target.get_window().id();

        let physical_size = logical_size.to_physical(window_target.get_window().get_hidpi_factor());
        window_target.update_size(physical_size.width as u16, physical_size.height as u16);
        
        let mut window = Window::new(window_target);
        window.set_root_view(view_data);
        self.windows.insert(window_id, window);

        Ok(window_id)
    }

    pub fn add_window_view_model<V: View>(&mut self,
        window_builder: winit::WindowBuilder, view_model: &Rc<RefCell<V>>) -> Result<winit::WindowId> {
        self.add_window(window_builder, V::create_view(view_model))
    }

    pub fn get_title(&self) -> &'static str {
        self.title
    }

    pub fn get_drawing_context(&self) -> Rc<RefCell<DrawingContext>> {
        self.drawing_context.clone()
    }

    pub fn create_loop_proxy(&self) -> winit::EventsLoopProxy {
        self.events_loop.create_proxy()
    }

    pub fn get_events_loop_interation(&mut self) -> &mut Event<()> {
        &mut self.events_loop_interation
    }

    pub fn run(&mut self) {
        let mut running = true;
        let mut frame_no = 0;

        let events_loop = &mut self.events_loop;
        let events_loop_interation = &mut self.events_loop_interation;
        let event_processor = &mut self.event_processor;
        let drawing_context = self.drawing_context.clone();
        let windows = &mut self.windows;

        events_loop.run_forever(|event| {
            if let winit::Event::WindowEvent { ref window_id, ref event } = event {
                if let Some(window) = windows.get_mut(window_id) {
                    match event {
                        winit::WindowEvent::CloseRequested => {
                            running = false;
                        },

                        winit::WindowEvent::Refresh => {
                            if let Some(ref mut root_view) = window.get_root_view_mut() {
                                let mut root_control = root_view.view_data.root_control.borrow_mut();
                                root_control.set_is_dirty(true);
                            }
                        },

                        winit::WindowEvent::Resized(logical_size) => {
                            let physical_size = logical_size.to_physical(window.get_drawing_target().get_window().get_hidpi_factor());

                            let drawing_context = &mut drawing_context.borrow_mut();
                            drawing_context.update_size(window.get_drawing_target_mut(),
                                physical_size.width as u16, physical_size.height as u16);

                            if let Some(ref mut root_view) = window.get_root_view_mut() {
                                let size = Size::new(physical_size.width as f32, physical_size.height as f32);
                                let mut root_control = root_view.view_data.root_control.borrow_mut();
                                let _ = root_control.get_preferred_size(drawing_context, size);
                                root_control.set_rect(Rect::new(0f32, 0f32, size.width, size.height));
                            }
                        },
                        
                        _ => ()
                    }

                    event_processor.handle_event(window, event);
                }
            };

            events_loop_interation.emit(());
            CallbackExecutor::execute_all_in_queue();
            Dispatcher::execute_all_in_queue();

            for window in windows.values_mut() {
                let logical_size = window.get_drawing_target().get_window().get_inner_size().unwrap_or(LogicalSize::new(0.0, 0.0));
                let physical_size = logical_size.to_physical(window.get_drawing_target().get_window().get_hidpi_factor());
                if running && physical_size.width > 0.0 && physical_size.height > 0.0 {
                    if let Some(ref mut root_view) = window.get_root_view_mut() {
                        let root_control = root_view.view_data.root_control.borrow();
                        if root_control.is_dirty() {
                            frame_no += 1;
                            println!("Frame no: {}", frame_no);
                        }
                    }

                    let need_swap_buffers = Application::render(window, &mut drawing_context.borrow_mut(),
                        physical_size.width as u32, physical_size.height as u32);
                    window.set_need_swap_buffers(need_swap_buffers);
                }
            }

            for window in windows.values_mut() {
                if window.get_need_swap_buffers() {
                    window.get_drawing_target_mut().swap_buffers();
                }
            }

            if running { winit::ControlFlow::Continue } else { winit::ControlFlow::Break }
        });
    }

    fn render(window: &mut Window,
        drawing_context: &mut DrawingContext,
        width: u32, height: u32) -> bool {
        let (drawing_target, root_view) = window.get_drawing_target_and_root_view_mut();
        if let Some(ref mut root_view) = root_view {
            let mut root_control = root_view.view_data.root_control.borrow_mut();

            if root_control.is_dirty() {
                let size = Size::new(width as f32, height as f32);
                let _ = root_control.get_preferred_size(drawing_context, size);
                root_control.set_rect(Rect::new(0f32, 0f32, size.width, size.height));

                let primitives = root_control.to_primitives(drawing_context);

                let res = drawing_context.begin(drawing_target);
                if let Err(err) = res {
                    eprintln!("Render error on begin drawing: {}", err);
                } else {
                    drawing_context.clear(drawing_target.get_render_target(), &[0.5f32, 0.4f32, 0.3f32, 1.0f32]);
                    let res = drawing_context.draw(drawing_target.get_render_target(),
                        PhysPixelSize::new(width as f32, height as f32),
                        primitives);
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

impl Drop for Application {
    fn drop(&mut self) {
        // It is important to drop windows before drawing_context!
        // Windows cleanup graphics resources and drawing context drops graphics device.
        self.windows.clear();
    }
}
