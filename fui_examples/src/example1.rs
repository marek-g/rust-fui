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
    pub counter2: Property<i32>
}

impl MainViewModel {
    pub fn new() -> Self {
        MainViewModel { counter: Property::new(10), counter2: Property::new(0) }
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
        // controls
        let btn1 = Button::control(Text::control("Decrease"));
        let btn2 = Button::control(Text::control("Increase"));
        let text1 = Text::control("");
        let text2 = Text::control("");

        // events
        btn1.borrow_mut().data.events.clicked.set_vm(view_model, |vm, _| { vm.decrease(); });
        btn2.borrow_mut().data.events.clicked.set_vm(view_model, |vm, _| { vm.increase(); });

        // bindings
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();
        let bindings = vec![
            text1.borrow_mut().data.properties.text.bind_c(&mut vm.counter, |counter| { format!("Counter {}", counter) } ),
            text2.borrow_mut().data.properties.text.bind_c(&mut vm.counter2, |counter| { format!("Counter2 {}", counter) } ),

            // test for two way binding            
            vm.counter2.bind(&mut vm.counter),
            vm.counter.bind(&mut vm.counter2),
        ];

        // layout
        let root_control = Horizontal::control(vec![
            text1, btn1, btn2, text2
        ]);

        ViewData {
            root_control: root_control,
            bindings: bindings,
        }
    }
}

fn main() {
    let mut app = Application::new("Marek Ogarek").unwrap();

    let window_builder1 = winit::WindowBuilder::new().with_title("Window 1");
    let main_view_model = Rc::new(RefCell::new(MainViewModel::new()));
    app.add_window_view_model(window_builder1, &main_view_model).unwrap();

    /*let window_builder2 = winit::WindowBuilder::new().with_title("Window 2");
    let main_view_model2 = Rc::new(RefCell::new(MainViewModel::new()));
    app.add_window_view_model(window_builder2, &main_view_model2).unwrap();*/

    app.run();
}
