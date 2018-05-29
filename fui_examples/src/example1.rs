#![windows_subsystem = "windows"]

extern crate fui;

use fui::application::*;
use fui::control::*;
use fui::controls::*;
use fui::layout::*;
use fui::*;

use std::cell::RefCell;
use std::rc::Rc;

struct MainViewModel {
    pub counter: i32
}

impl MainViewModel {
    pub fn new() -> Self {
        MainViewModel { counter: 10 }
    }

    pub fn increase(&mut self) {
        println!("increase!");
        self.counter += 1;
    }

    pub fn decrease(&mut self) {
        println!("decrease!");
        self.counter -= 1;
    }
}

impl View for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<MainViewModel>>) -> ViewData {
        let mut btn1 = Button::new(Text::new("Decrease".to_string()));
        let self_rc = view_model.clone();
        btn1.events.clicked.set(move |_| { self_rc.borrow_mut().decrease(); });

        let mut btn2 = Button::new(Text::new("Increase".to_string()));
        let self_rc = view_model.clone();
        btn2.events.clicked.set(move |_| { self_rc.borrow_mut().increase(); });

        let text1 = Text::new("Count: 0".to_string());

        let root_control = Horizontal::new(vec![
            text1, btn1, btn2
        ]);

        ViewData {
            root_control: root_control,
        }
    }
}

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let mut main_view_model = Rc::new(RefCell::new(MainViewModel::new()));
    app.set_root_view_model(&main_view_model);

    //let main_view = main_view_model.create_view();
    //app.set_root_control(main_view);

    //app.set_root_control(btn1);
    //app.clear_root_control();

    //let mut v: Vec<Box<ControlObject>> = vec![];
    //v.push(Button::new());

    app.run();
}
