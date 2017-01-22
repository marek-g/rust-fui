extern crate winit;

use backend::gfx_application::GFXApplication;
use render::primitive::Primitive;

pub struct Application {
    application: GFXApplication
}

impl Application {
    pub fn new(window_builder: winit::WindowBuilder) -> Self {
        Application {
            application: GFXApplication::new(window_builder)
        }
    }

    pub fn poll_events(&self) -> winit::PollEventsIterator {
        self.application.poll_events()
    }

    pub fn get_render_width(&self) -> f32 {
        self.application.get_render_width()
    }

    pub fn get_render_height(&self) -> f32 {
        self.application.get_render_height()
    }

    pub fn draw_primitives(&mut self, primitives: Vec<Primitive>,
                           width: f32, height: f32) {
        self.application.draw_primitives(primitives, width, height)
    }

    pub fn text_width(&mut self, size: f32, text: &str) -> f32 {
        self.application.text_width(size, text)
    }
}