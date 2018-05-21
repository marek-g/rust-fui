extern crate winit;

use drawing::units::{ PhysPixelSize };

use drawing_context::DrawingContext;
use control::*;
use common::*;
use events::*;

pub struct Application {
    title: &'static str,
    events_loop: winit::EventsLoop,
    event_processor: EventProcessor,
    drawing_context: DrawingContext,
    root_control: Option<Box<ControlObject>>,
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
            root_control: None,
        }
    }

    pub fn set_root_control(&mut self, root_control: Box<ControlObject>) {
        self.root_control = Some(root_control);
    }

    pub fn clear_root_control(&mut self) {
        self.root_control = None;
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
                let root_control = &mut self.root_control;

                events_loop.poll_events(|event| {
                    if let winit::Event::WindowEvent { ref event, .. } = event {
                        match event {
                            winit::WindowEvent::Closed => {
                                running = false;
                            },
                            winit::WindowEvent::Resized(ref w, ref h) => {
                                width = *w; height = *h;
                                if let Some(ref mut root_control) = root_control {
                                    let size = Size::new(*w as f32, *h as f32);
                                    let control_size = root_control.get_preferred_size(drawing_context, size);
                                    root_control.set_rect(Rect::new(0f32, 0f32, *w as f32, *h as f32));
                                }
                                drawing_context.update_window_size(*w as u16, *h as u16)
                            },
                            _ => ()
                        }
                    };

                    if let Some(ref mut root_control) = root_control {
                        event_processor.handle_event(root_control, &event);
                    }
                });
            }

            if !running { return }

            if width <= 0 || height <= 0 { continue }

            self.render(width, height);
        }
    }

    fn render(&mut self, width: u32, height: u32) {
        if let Some(ref mut root) = self.root_control {
            let primitives = root.to_primitives(&mut self.drawing_context);

            self.drawing_context.draw(PhysPixelSize::new(width as f32, height as f32),
                primitives);
        }
    }
}
