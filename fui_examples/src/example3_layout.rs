#![windows_subsystem = "windows"]

extern crate fui;
extern crate fui_macros;
extern crate typed_builder;
extern crate typemap;
extern crate winit;

use fui::application::*;
use fui::controls::*;
use fui::layout::*;
use fui::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::Rc;

use typemap::TypeMap;

struct MainViewModel {
    pub counter: Property<i32>,
    pub counter2: Property<i32>,
}

impl MainViewModel {
    pub fn new() -> Self {
        MainViewModel {
            counter: Property::new(10),
            counter2: Property::new(0),
        }
    }

    pub fn increase(&mut self) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(&mut self) {
        self.counter.change(|c| c - 1);
    }
}

impl View for MainViewModel {
    fn to_view(self, _context: ViewContext) -> Rc<RefCell<ControlObject>> {
        let view_model = &Rc::new(RefCell::new(self));
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();

        let root_control = ui!(
            Grid {
                columns: 2,

                Text { text: (&vm.counter, |counter| format!("Counter {}", counter)) },
                Button {
                    clicked: Callback::new(view_model, |vm, _| vm.decrease()),
                    Text { text: "Decrease" },
                },
                Button {
                    clicked: Callback::new(view_model, |vm, _| vm.increase()),
                    Text { text: "Increase" },
                },
                Text { text: (&vm.counter2, |counter| format!("Counter2 {}", counter)) },
            }
        );

        vm.counter2.bind(&mut vm.counter);
        vm.counter.bind(&mut vm.counter2);

        root_control
    }
}

fn main() {
    let mut app = Application::new("Marek Ogarek").unwrap();

    let main_view_model = MainViewModel::new();

    {
        let mut window_manager = app.get_window_manager().borrow_mut();
        let window_builder = winit::WindowBuilder::new().with_title("Window 1");
        window_manager
            .add_window_view_model(window_builder, app.get_events_loop(), main_view_model)
            .unwrap();
    }

    app.run();
}
