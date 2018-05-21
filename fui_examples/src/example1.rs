#![windows_subsystem = "windows"]

extern crate fui;

use fui::application::*;
use fui::control::*;
use fui::controls::button::*;
use fui::controls::text::*;
use fui::layout::horizontal::*;

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let mut btn1 = Button::new(Text::new("Marek".to_string()));
    let event_subscription = btn1.events.clicked.subscribe(|_| { println!("clicked!") });

    let text1 = Text::new("Label".to_string());
    
    app.set_root_control(Horizontal::new(vec![
        text1, btn1
    ]));

    //app.set_root_control(btn1);
    //app.clear_root_control();

    //let mut v: Vec<Box<ControlObject>> = vec![];
    //v.push(Button::new());

    app.run();
}
