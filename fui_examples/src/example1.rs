#![windows_subsystem = "windows"]

extern crate fui;

use fui::application::*;
use fui::control::*;
use fui::controls::*;
use fui::layout::*;

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let mut btn1 = Button::new(Text::new("Marek".to_string()));
    let event_subscription1 = btn1.events.clicked.subscribe(|_| { println!("clicked 1!") });

    let mut btn2 = Button::new(Text::new("Marek 2".to_string()));
    let event_subscription2 = btn1.events.clicked.subscribe(|_| { println!("clicked 2!") });

    let text1 = Text::new("Label".to_string());
    
    app.set_root_control(Horizontal::new(vec![
        text1, btn1, btn2
    ]));

    //app.set_root_control(btn1);
    //app.clear_root_control();

    //let mut v: Vec<Box<ControlObject>> = vec![];
    //v.push(Button::new());

    app.run();
}
