#![windows_subsystem = "windows"]

extern crate fui;

use fui::application::*;
use fui::control::*;
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
        let mut btn1 = Button::new(Text::new("Decrease".to_string()));
        let mut btn2 = Button::new(Text::new("Increase".to_string()));
        let mut text1 = Text::new("".to_string());
        let mut text2 = Text::new("".to_string());

        // events
        btn1.events.clicked.set_vm(view_model, |vm, _| { vm.decrease(); });
        btn2.events.clicked.set_vm(view_model, |vm, _| { vm.increase(); });

        // bindings
        let mut vm: &mut MainViewModel = &mut view_model.borrow_mut();
        let bindings = vec![
            text1.properties.text.bindc(&mut vm.counter, |counter| { format!("Counter {}", counter) } ),
            text2.properties.text.bindc(&mut vm.counter2, |counter| { format!("Counter2 {}", counter) } ),

            // test for two way binding            
            vm.counter2.bind(&mut vm.counter),
            vm.counter.bind(&mut vm.counter2),
        ];

        // layout
        let root_control = Horizontal::new(vec![
            text1, btn1, btn2, text2
        ]);

        ViewData {
            root_control: root_control,
            bindings: bindings,
        }
    }
}

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let main_view_model = Rc::new(RefCell::new(MainViewModel::new()));
    app.set_root_view_model(&main_view_model);

    //let main_view = main_view_model.create_view();
    //app.set_root_control(main_view);

    //app.set_root_control(btn1);
    //app.clear_root_control();

    //let mut v: Vec<Box<ControlObject>> = vec![];
    //v.push(Button::new());

    app.run();
}
