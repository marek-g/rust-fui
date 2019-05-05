#![windows_subsystem = "windows"]

extern crate fui;
extern crate winit;

use fui::application::*;
use fui::controls::*;
use fui::layout::*;
use fui::*;

use std::cell::RefCell;
use std::rc::Rc;

use Property;

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
    fn create_view(view_model: &Rc<RefCell<MainViewModel>>) -> ViewData {
        let mut bindings = Vec::<fui::EventSubscription>::new();

        let root_control = Horizontal::control(vec![
            Text::control("").with_binding(&mut bindings, view_model, |vm, btn| {
                btn.data
                    .properties
                    .text
                    .bind_c(&mut vm.counter, |counter| format!("Counter {}", counter))
            }),
            Button::control(Text::control("Decrease")).with_vm(view_model, |vm, btn| {
                btn.data.events.clicked.set_vm(vm, |vm, _| {
                    vm.decrease();
                })
            }),
            Button::control(Text::control("Increase")).with_vm(view_model, |vm, btn| {
                btn.data.events.clicked.set_vm(vm, |vm, _| {
                    vm.increase();
                })
            }),
            Text::control("").with_binding(&mut bindings, view_model, |vm, btn| {
                btn.data
                    .properties
                    .text
                    .bind_c(&mut vm.counter2, |counter| format!("Counter2 {}", counter))
            }),
        ]);

        let vm: &mut MainViewModel = &mut view_model.borrow_mut();
        bindings.push(vm.counter2.bind(&mut vm.counter));
        bindings.push(vm.counter.bind(&mut vm.counter2));

        ViewData {
            root_control: root_control,
            bindings: bindings,
        }
    }
}

fn main() {
    let mut app = Application::new("Marek Ogarek").unwrap();

    let main_view_model = Rc::new(RefCell::new(MainViewModel::new()));

    {
        let mut window_manager = app.get_window_manager().borrow_mut();
        let window_builder = winit::WindowBuilder::new().with_title("Window 1");
        window_manager
            .add_window_view_model(window_builder, app.get_events_loop(), &main_view_model)
            .unwrap();
    }

    app.run();
}
