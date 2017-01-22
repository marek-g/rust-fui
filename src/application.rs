use backend::renderer::*;
use common::size::*;
use controls::control::*;
use render::conversion::*;

pub struct Application {
    backend_app: ::backend::application::Application,
    renderer: Renderer,

    root_control: Option<Box<Control>>,
}



impl Application {
    pub fn new(title : &'static str) -> Self {
        Application {
            backend_app: ::backend::application::Application::new(&title),
            renderer: Renderer::new(),
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
        self.backend_app.run();
    }

    /*fn render(&mut self, args: &RenderArgs) {
        match self.root_control {
            Some(ref mut root) => {
                let control_size = root.get_preferred_size(Size::new(args.width as f32, args.height as f32),
                    &mut self.renderer);
                root.set_size(control_size, &mut self.renderer);
                let primitives = convert_control_to_primitives(&**root);
                self.renderer.draw_primitives(args, primitives);
            },
            _ => {}
        }
    }*/
}
