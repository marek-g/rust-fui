extern crate piston;
extern crate piston_window;
extern crate opengl_graphics;

use self::piston_window::PistonWindow;
use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::opengl_graphics::OpenGL;

use backend::opengl_renderer::*;
use controls::control::*;
use render::conversion::*;

pub struct Application {
    main_window: PistonWindow,
    renderer: OpenGLRenderer,

    root_control: Option<Box<Control>>,

    rotation: f64
}

impl Application {
    pub fn new(title : &'static str) -> Self {
        let opengl_version = OpenGL::V3_2;

        let window : PistonWindow = WindowSettings::new(
            title,
            [800, 600]
        )
            .opengl(opengl_version)
            .decorated(true)
            .resizable(true)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

        Application {
            main_window: window,
            renderer: OpenGLRenderer::new(opengl_version),
            root_control: None,
            rotation: 0.0
        }
    }

    pub fn set_root_control(&mut self, root_control: Box<Control>) {
        self.root_control = Some(root_control);
    }

    pub fn clear_root_control(&mut self) {
        self.root_control = None;
    }

    pub fn run(&mut self) {
        let mut events = self.main_window.events();
        while let Some(e) = events.next(&mut self.main_window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        match self.root_control {
            Some(ref root) => {
                let primitives = convert_control_to_primitives(&**root);
                self.renderer.draw_primitives(args, primitives);
            },
            _ => {}
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 1.0 * args.dt;
    }
}
