extern crate winit;

use std::cell::RefCell;
use std::rc::Rc;
use drawing::units::{ PhysPixelSize };

use drawing_context::DrawingContext;
use common::*;
use events::*;
use View;
use ViewData;

pub struct Application {
    title: &'static str,
    events_loop: winit::EventsLoop,
    event_processor: EventProcessor,
    drawing_context: DrawingContext,
    root_view: Option<ViewData>,
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
        }
    }

    pub fn get_title(&self) -> &'static str {
        self.title
    }

    pub fn set_root_view(&mut self, root_view: ViewData) {
        self.root_view = Some(root_view);
    }

    pub fn set_root_view_model<V: View>(&mut self, view_model: &Rc<RefCell<V>>) {
        self.root_view = Some(V::create_view(view_model));
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
                                    let _ = root_view.root_control.get_preferred_size(drawing_context, size);
                                    root_view.root_control.set_rect(Rect::new(0f32, 0f32, size.width, size.height));
                                }
                                drawing_context.update_window_size(*w as u16, *h as u16)
                            },
                            _ => ()
                        }
                    };

                    if let Some(ref mut root_view) = root_view {
                        event_processor.handle_event(&mut root_view.root_control, &event);
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
            let dirty = true;
            if dirty {
                let size = Size::new(width as f32, height as f32);
                let _ = root_view.root_control.get_preferred_size(&mut self.drawing_context, size);
                root_view.root_control.set_rect(Rect::new(0f32, 0f32, size.width, size.height));
            }
            let primitives = root_view.root_control.to_primitives(&mut self.drawing_context);

            self.drawing_context.draw(PhysPixelSize::new(width as f32, height as f32),
                primitives);
        }
    }
}
