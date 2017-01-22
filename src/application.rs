extern crate winit;

use backend::renderer::*;
use common::size::*;
use controls::control::*;

pub struct Application {
    title: &'static str,
    backend_app: ::backend::application::Application,
    root_control: Option<Box<Control>>,
}

impl Application {
    pub fn new(title: &'static str) -> Self {
        let window_builder = winit::WindowBuilder::new()
            .with_title(title);

        Application {
            title: title,
            backend_app: ::backend::application::Application::new(window_builder),
            root_control: None,
        }
    }

    pub fn set_root_control(&mut self, root_control: Box<Control>) {
        self.root_control = Some(root_control);
    }

    pub fn clear_root_control(&mut self) {
        self.root_control = None;
    }

    pub fn run(&mut self) {
        'main: loop {
            for event in self.backend_app.poll_events() {
                match event {
                    winit::Event::Closed => return,
                    _ => (),
                }
            }

            let width = self.backend_app.get_render_width();
            let height = self.backend_app.get_render_height();
            self.render(width, height);
        }
    }

    fn render(&mut self, width: f32, height: f32) {
        //let mut test = &mut self;
        if let Some(ref mut root) = self.root_control {
            let control_size = root.get_preferred_size(Size::new(width, height),
                                                       &mut self.backend_app);
            root.set_size(control_size, &mut self.backend_app);
            let primitives = root.to_primitives();

            self.backend_app.draw_primitives(primitives, width, height);
        }
    }

    pub fn text_width(&self, size: f32, text: &str) -> f32 {
        //self.backend_app.text_width(size, text)
        1.0
    }
}
