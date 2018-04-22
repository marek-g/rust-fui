#![windows_subsystem = "windows"]

extern crate fui;

use fui::application::*;
use fui::controls::button::*;
use fui::controls::control::*;

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let btn1 = Button::new();
    app.set_root_control(btn1);
    //app.clear_root_control();

    let mut v: Vec<Box<Control>> = vec![];
    v.push(Box::new(Button::new()));

    app.run();
}
