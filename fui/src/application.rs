extern crate winit;

use drawing::backend::*;
use drawing::units::*;

use drawing_context::DrawingContext;
use controls::control::*;

pub struct Application<C: Control> {
    title: &'static str,
    events_loop: winit::EventsLoop,
    drawing_context: DrawingContext,
    root_control: Option<C>,
}

impl<C: Control> Application<C> {
    pub fn new(title: &'static str) -> Self {
        ::high_dpi::set_process_high_dpi_aware();

        let window_builder = winit::WindowBuilder::new()
            .with_title(title);
        let events_loop = winit::EventsLoop::new();
        let drawing_context = DrawingContext::create(window_builder, &events_loop);

        Application {
            title: title,
            events_loop: events_loop,
            drawing_context: drawing_context,
            root_control: None,
        }
    }

    pub fn set_root_control(&mut self, root_control: C) {
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
                let drawing_context = &mut self.drawing_context;

                events_loop.poll_events(|event| {
                    if let winit::Event::WindowEvent { event, .. } = event {
                        match event {
                            winit::WindowEvent::Closed => {
                                running = false;
                            },
                            winit::WindowEvent::Resized(w, h) => {
                            width = w; height = h;
                            drawing_context.update_window_size(w as u16, h as u16)
                            },
                            _ => (),
                        }
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
            let control_size = root.get_preferred_size(
                ::common::size::Size::new(width as f32, height as f32),
                &mut self.drawing_context);
            let primitives = root.to_primitives(control_size, &mut self.drawing_context);

            self.drawing_context.draw(PhysPixelSize::new(width as f32, height as f32),
                primitives);
        }
    }
}
