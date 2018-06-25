extern crate winit;

use std::cell::RefCell;
use std::rc::Rc;

use drawing::units::{ PhysPixelSize };
use drawing_context::DrawingContext;
use common::*;
use events::*;
use View;
use ViewData;
use RootView;

pub struct Application {
    title: &'static str,
    events_loop: winit::EventsLoop,
    event_processor: EventProcessor,
    drawing_context: DrawingContext,
    root_view: Option<RootView>,
    frame_no: i32
}

impl Application {
    pub fn new(title: &'static str) -> Self {
        ::high_dpi::set_process_high_dpi_aware();

        let window_builder = winit::WindowBuilder::new()
            .with_title(title);
        let events_loop = winit::EventsLoop::new();
        let drawing_context = DrawingContext::create(window_builder, &events_loop);

        Application {
            title: title,
            events_loop: events_loop,
            event_processor: EventProcessor::new(),
            drawing_context: drawing_context,
            root_view: None,
            frame_no: 0,
        }
    }

    pub fn get_title(&self) -> &'static str {
        self.title
    }

    pub fn set_root_view(&mut self, view_data: ViewData) {
        self.root_view = Some(RootView::new(view_data));
    }

    pub fn set_root_view_model<V: View>(&mut self, view_model: &Rc<RefCell<V>>) {
        self.set_root_view(V::create_view(view_model));
    }

    pub fn clear_root(&mut self) {
        self.root_view = None;
    }

    pub fn run(&mut self) {
        let mut width = 0;
        let mut height = 0;

        let mut running = true;

        while running {
            {
                let events_loop = &mut self.events_loop;
                let event_processor = &mut self.event_processor;
                let drawing_context = &mut self.drawing_context;
                let root_view = &mut self.root_view;

                events_loop.poll_events(|event| {
                    if let winit::Event::WindowEvent { ref event, .. } = event {
                        match event {
                            winit::WindowEvent::Closed => {
                                running = false;
                            },
                            winit::WindowEvent::Resized(ref w, ref h) => {
                                width = *w; height = *h;
                                if let Some(ref mut root_view) = root_view {
                                    let size = Size::new(*w as f32, *h as f32);
                                    let mut root_control = root_view.view_data.root_control.borrow_mut();
                                    let _ = root_control.get_preferred_size(drawing_context, size);
                                    root_control.set_rect(Rect::new(0f32, 0f32, size.width, size.height));

                                    root_control.set_is_dirty(true);
                                }
                                drawing_context.update_window_size(*w as u16, *h as u16)
                            },
                            _ => ()
                        }
                    };

                    if let Some(ref mut root_view) = root_view {
                        event_processor.handle_event(root_view, &event);
                    }
                });
            }

            if !running { return }

            if width <= 0 || height <= 0 { continue }

            self.render(width, height);
        }
    }

    fn render(&mut self, width: u32, height: u32) {
        if let Some(ref mut root_view) = self.root_view {
            let mut root_control = root_view.view_data.root_control.borrow_mut();

            if root_control.is_dirty() {
                let size = Size::new(width as f32, height as f32);
                let _ = root_control.get_preferred_size(&mut self.drawing_context, size);
                root_control.set_rect(Rect::new(0f32, 0f32, size.width, size.height));

                let primitives = root_control.to_primitives(&mut self.drawing_context);
                self.drawing_context.draw(PhysPixelSize::new(width as f32, height as f32),
                    primitives);

                self.frame_no += 1;
                println!("Frame no: {}", self.frame_no);

                root_control.set_is_dirty(false);
            }
        }
    }
}
