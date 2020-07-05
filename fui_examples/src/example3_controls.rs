#![windows_subsystem = "windows"]

use fui::*;
use fui_app::*;
use fui_controls::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::Rc;

use typemap::TypeMap;
use winit::window::WindowBuilder;

struct MainViewModel {
    pub text: Property<String>,
    pub text2: Property<String>,
    pub progress: Property<f32>,
    pub counter: Property<i32>,
    pub counter2: Property<i32>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MainViewModel {
            text: Property::new("My text"),
            text2: Property::new("ąęść"),
            progress: Property::new(0.5f32),
            counter: Property::new(10),
            counter2: Property::new(0),
        }))
    }

    pub fn increase(&mut self) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(&mut self) {
        self.counter.change(|c| c - 1);
    }
}

impl ViewModel for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();

        vm.counter2.bind(&mut vm.counter);
        vm.counter.bind(&mut vm.counter2);

        let radio1 = ui!(ToggleButton { Style: Tab {}, Text { text: "Radio 1"} });
        let radio2 = ui!(ToggleButton { Style: Tab {}, Text { text: "Radio 2"} });
        let radio3 = ui!(ToggleButton { Style: Tab {}, Text { text: "Radio 3"} });
        let radio_controller =
            RadioController::new(vec![radio1.clone(), radio2.clone(), radio3.clone()]);

        let radio4 = ui!(ToggleButton { Style: Radio {}, Text { text: "Radio 4"} });
        let radio5 = ui!(ToggleButton { Style: Radio {}, Text { text: "Radio 5"} });
        let radio6 = ui!(ToggleButton { Style: Radio {}, Text { text: "Radio 6"} });
        let radio_controller2 =
            RadioController::new(vec![radio4.clone(), radio5.clone(), radio6.clone()]);

        let content = ui!(
            TabControl {
                Grid {
                    Title: "Tab 1",

                    columns: 2,
                    default_height: Length::Auto,

                    TextBox {
                        text: &mut vm.text,
                    },
                    Text {
                        text: &vm.text,
                    },

                    TextBox {
                        text: &mut vm.text2,
                    },
                    Text {
                        Style: Default {
                            color: [1.0f32, 0.8f32, 0.0f32, 1.0f32],
                        },
                        text: &vm.text2,
                    },

                    ScrollBar {
                        orientation: Orientation::Horizontal,
                        value: &mut vm.progress,
                    },
                    ProgressBar {
                        value: &vm.progress,
                    },

                    DropDown {
                        Column: 0,
                        Row: 3,

                        Text { text: "Element 1"},
                        Text { text: "Element 2"},
                        Text { text: "Element 3"},
                    },

                    Vertical {
                        Column: 0,
                        Row: 4,

                        @radio1,
                        @radio2,
                        @radio3,
                    },
                    Vertical {
                        @radio4,
                        @radio5,
                        @radio6,
                    },

                    Vertical {
                        Column: 0,
                        Row: 5,

                        ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 1"} },
                        ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 2"} },
                        ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 3"} },
                    },
                },

                Grid {
                    Title: "Tab 2",

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
            }
        );

        let data_holder = DataHolder {
            data: (radio_controller, radio_controller2),
        };
        data_holder.to_view(
            None,
            ViewContext {
                attached_values: TypeMap::new(),
                children: Box::new(vec![content as Rc<RefCell<dyn ControlObject>>]),
            },
        )
    }
}

fn main() -> Result<()> {
    let mut app = Application::new("Example: layout").unwrap();

    app.add_window(
        WindowBuilder::new().with_title("Example: layout"),
        MainViewModel::new(),
    )?;

    app.run();

    Ok(())
}
