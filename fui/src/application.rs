extern crate winit;

use drawing::backend::*;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::units::*;
use drawing_gfx::backend::{ GfxWindowBackend, GfxTexture, GfxResources, GfxFactory };
use drawing_gfx::font_gfx_text::GfxTextFont;
use common::size::*;
use controls::control::*;

pub struct Application {
    title: &'static str,
    events_loop: winit::EventsLoop,
    resources: Resources<GfxTexture, GfxTextFont<GfxResources, GfxFactory>>,
    renderer: Renderer<GfxWindowBackend>,
    root_control: Option<Box<Control>>,
}

impl Application {
    pub fn new(title: &'static str) -> Self {
        let window_builder = winit::WindowBuilder::new()
            .with_title(title);
        let mut events_loop = winit::EventsLoop::new();

        Application {
            title: title,
            events_loop: events_loop,
            resources: Resources::new(),
            renderer: Renderer::new(GfxWindowBackend::create_window_backend(window_builder, &events_loop)),
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
        let mut width = 0;
        let mut height = 0;

        'main: loop {
            self.events_loop.poll_events(|event| {
                if let winit::Event::WindowEvent { event, .. } = event {
                    match event {
                        winit::WindowEvent::Closed => return,
                        winit::WindowEvent::Resized(w, h) => {
                           width = w; height = h;
                           self.renderer.update_window_size(w as u16, h as u16)
                        },
                        _ => (),
                    }
                }
            });

            if width <= 0 || height <= 0 { continue }

            self.render(width, height);
        }
    }

    fn render(&mut self, width: u32, height: u32) {
        //let mut test = &mut self;
        if let Some(ref mut root) = self.root_control {
            let control_size = root.get_preferred_size(::common::size::Size::new(width as f32, height as f32),
                                                       &mut self);
            root.set_size(control_size, &mut self);
            let primitives = root.to_primitives();

            self.renderer.draw(PhysPixelSize::new(width as f32, height as f32),
                primitives, &mut self.resources);
        }
    }

    pub fn text_width(&self, size: f32, text: &str) -> f32 {
        //self.backend_app.text_width(size, text)
        1.0
    }
}
