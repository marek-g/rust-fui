#![windows_subsystem = "windows"]

use anyhow::Result;
use fui_app::*;
use fui_controls::*;
use fui_core::*;
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
    pub drop_down_selected_item: Property<Option<Rc<RefCell<StringViewModel>>>>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MainViewModel {
            text: Property::new("My text"),
            text2: Property::new("ąęść"),
            progress: Property::new(0.5f32),
            counter: Property::new(10),
            counter2: Property::new(0),
            drop_down_selected_item: Property::new(None),
        }))
    }

    pub fn increase(&mut self) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(&mut self) {
        self.counter.change(|c| c - 1);
    }
}

type DropDown1 = DropDown<StringViewModel>;

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

        let tab1 = ui!(
            Grid {
                Title: "Tab 1",
                Margin: Thickness::all(8.0f32),

                columns: 2,
                default_height: Length::Auto,

                TextBox {
                    text: &mut vm.text,
                },
                Text {
                    Margin: Thickness::left(5.0f32),
                    text: &vm.text,
                },

                TextBox {
                    Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),
                    text: &mut vm.text2,
                },
                Text {
                    Margin: Thickness::new(5.0f32, 5.0f32, 0.0f32, 0.0f32),
                    Style: Default {
                        color: [1.0f32, 0.8f32, 0.0f32, 1.0f32],
                    },
                    text: &vm.text2,
                },

                ScrollBar {
                    Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),
                    orientation: Orientation::Horizontal,
                    value: &mut vm.progress,
                },
                ProgressBar {
                    Margin: Thickness::new(5.0f32, 5.0f32, 0.0f32, 0.0f32),
                    value: &vm.progress,
                },

                DropDown1 {
                    Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),
                    Column: 0,
                    Row: 3,

                    selected_item: &mut vm.drop_down_selected_item,
                    items: vec![
                        StringViewModel::new("Element A"),
                        StringViewModel::new("Element B"),
                        StringViewModel::new("Element C"),
                        StringViewModel::new("Element D"),
                        StringViewModel::new("Element E"),
                    ],
                },
                Text {
                    Margin: Thickness::new(5.0f32, 5.0f32, 0.0f32, 0.0f32),
                    text: (&vm.drop_down_selected_item, |vm: Option<Rc<RefCell<StringViewModel>>>| match &vm {
                        None => "-".to_string(),
                        Some(vm) => vm.borrow().text.clone(),
                    }),
                },

                Vertical {
                    Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),
                    Column: 0,
                    Row: 4,

                    radio1,
                    radio2,
                    radio3,
                },
                Vertical {
                    Margin: Thickness::new(5.0f32, 5.0f32, 0.0f32, 0.0f32),
                    radio4,
                    radio5,
                    radio6,
                },

                Vertical {
                    Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),
                    Column: 0,
                    Row: 5,

                    ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 1"} },
                    ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 2"} },
                    ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 3"} },
                },
            }
        );

        let tab2 = ui!(
            Grid {
                Title: "Tab 2",

                columns: 2,

                Text {
                    text: (&vm.counter, |counter| format!("Counter {}", counter))
                },
                Button {
                    VerticalAlignment: Alignment::Stretch,
                    clicked: Callback::new(view_model, |vm, _| vm.decrease()),
                    Text { text: "Decrease" },
                },
                Button {
                    VerticalAlignment: Alignment::Stretch,
                    clicked: Callback::new(view_model, |vm, _| vm.increase()),
                    Text { text: "Increase" },
                },
                Text {
                    text: (&vm.counter2, |counter| format!("Counter2 {}", counter))
                },
            }
        );

        let mut exit_callback = Callback::empty();
        exit_callback.set(|_| {
            println!("Exit");
        });

        let menu_items = vec![
            MenuItem::folder(
                "File",
                vec![
                    MenuItem::simple("Open...", Callback::empty()),
                    MenuItem::simple("Save...", Callback::empty()),
                    MenuItem::folder(
                        "Export",
                        vec![
                            MenuItem::simple("PDF...", Callback::empty()),
                            MenuItem::simple("PNG...", Callback::empty()),
                            MenuItem::simple("HTML...", Callback::empty()),
                        ],
                    ),
                    MenuItem::Separator,
                    MenuItem::simple("Exit", exit_callback),
                ],
            ),
            MenuItem::folder(
                "Help",
                vec![
                    MenuItem::simple("Help", Callback::empty()),
                    MenuItem::Separator,
                    MenuItem::simple("About", Callback::empty()),
                ],
            ),
        ];

        let content = ui!(Grid {
            rows: 2,
            heights: vec![(0, Length::Auto)],

            Menu { items: menu_items },

            TabControl {
                Margin: Thickness::new(8.0f32, 12.0f32, 8.0f32, 8.0f32),

                tab1,
                tab2
            }
        });

        let data_holder = DataHolder {
            data: (radio_controller, radio_controller2),
        };
        data_holder.to_view(
            None,
            ViewContext {
                attached_values: TypeMap::new(),
                children: Children::SingleStatic(content),
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
