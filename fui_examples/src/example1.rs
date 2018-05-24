#![windows_subsystem = "windows"]

extern crate fui;

use fui::application::*;
use fui::control::*;
use fui::controls::*;
use fui::layout::*;
use fui::*;

struct MainViewModel {
    pub counter: i32
}

impl MainViewModel {
    pub fn new() -> Self {
        MainViewModel { counter: 0 }
    }

    pub fn increase(&mut self) {
        self.counter += 1;
    }

    pub fn decrease(&mut self) {
        self.counter -= 1;
    }
}

impl View for MainViewModel {
    fn create_view(&mut self) -> Box<ControlObject> {
        let mut btn1 = Button::new(Text::new("Decrease".to_string()));
        let event_subscription1 = btn1.events.clicked.subscribe(|_| { println!("clicked 1!") });

        let mut btn2 = Button::new(Text::new("Increase".to_string()));
        let event_subscription2 = btn2.events.clicked.subscribe(|_| { println!("clicked 2!") });

        let text1 = Text::new("Count: 0".to_string());

        Horizontal::new(vec![
            text1, btn1, btn2
        ])
    }
}

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let mut main_view_model = MainViewModel::new();
    app.set_root_view_model(&mut main_view_model);

    //let main_view = main_view_model.create_view();
    //app.set_root_control(main_view);

    //app.set_root_control(btn1);
    //app.clear_root_control();

    //let mut v: Vec<Box<ControlObject>> = vec![];
    //v.push(Button::new());

    app.run();
}
