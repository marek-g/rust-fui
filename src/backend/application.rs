use backend::gfx_application::GFXApplication;

pub struct Application {
    application: GFXApplication
}

impl Application {
    pub fn new(title: &'static str) -> Self {
        Application {
            application: GFXApplication::new(title)
        }
    }

    pub fn run(&mut self) {
        self.application.run();
    }
}