//#![feature(windows_subsystem)]
//#![windows_subsystem = "windows"]

extern crate fui;

use fui::application::*;

fn main() {
    let mut app = Application::new("Marek Ogarek");
    app.run();
}
