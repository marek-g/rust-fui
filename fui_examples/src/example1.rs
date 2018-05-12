#![windows_subsystem = "windows"]

extern crate fui;

use fui::application::*;
use fui::controls::button::*;
use fui::controls::control::*;

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let mut btn1 = Box::new(Button::new());
    let event_subscription = btn1.events.clicked.subscribe(|()| { println!("clicked!") });
    app.set_root_control(btn1);
    //app.clear_root_control();

    let mut v: Vec<Box<ControlObject>> = vec![];
    v.push(Box::new(Button::new()));

    app.run();
}
