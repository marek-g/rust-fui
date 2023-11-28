#![windows_subsystem = "windows"]

use anyhow::{Error, Result};
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::Rc;
use tokio::task::LocalSet;

use typemap::TypeMap;

struct MainViewModel {
    pub window: Rc<RefCell<dyn WindowService>>,
    pub text: Property<String>,
    pub text2: Property<String>,
    pub progress: Property<f32>,
    pub is_busy: Property<bool>,
    pub counter: Property<i32>,
    pub counter2: Property<i32>,
    pub drop_down_selected_item: Property<Option<Rc<StringViewModel>>>,
}

impl MainViewModel {
    pub fn new(window: Rc<RefCell<dyn WindowService>>) -> Rc<Self> {
        Rc::new(MainViewModel {
            window,
            text: Property::new("My text"),
            text2: Property::new("ąęść"),
            progress: Property::new(0.5f32),
            is_busy: Property::new(false),
            counter: Property::new(10),
            counter2: Property::new(0),
            drop_down_selected_item: Property::new(None),
        })
    }

    pub fn increase(&self) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(&self) {
        self.counter.change(|c| c - 1);
    }
}

type DropDown1 = DropDown<StringViewModel>;

impl ViewModel for MainViewModel {
    fn create_view(self: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>> {
        self.counter2.bind(&self.counter);
        self.counter.bind(&self.counter2);

        let radio4 = ui!(ToggleButton { Style: Radio {}, Text { text: "Radio 4"} })
            as Rc<RefCell<dyn ControlObject>>;
        let radio5 = ui!(ToggleButton { Style: Radio {}, Text { text: "Radio 5"} })
            as Rc<RefCell<dyn ControlObject>>;
        let radio6 = ui!(ToggleButton { Style: Radio {}, Text { text: "Radio 6"} })
            as Rc<RefCell<dyn ControlObject>>;
        let radio_controller = RadioController::<StyledControl<ToggleButton>>::new(vec![
            radio4.clone(),
            radio5.clone(),
            radio6.clone(),
        ]);

        let tab1 = ui!(
            Grid {
                Title: "Tab 1",

                columns: 1,
                default_height: Length::Auto,

                Grid {
                    Margin: Thickness::all(8.0f32),

                    columns: 2,
                    default_height: Length::Auto,

                    TextBox {
                        text: self.text.clone(),
                    },
                    Text {
                        Margin: Thickness::left(5.0f32),
                        text: &self.text,
                    },

                    TextBox {
                        Style: Default {
                            password: true,
                        },
                        Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),
                        text: self.text2.clone(),
                    },
                    Text {
                        Style: Default {
                            color: [1.0f32, 0.8f32, 0.0f32, 1.0f32],
                        },
                        Margin: Thickness::new(5.0f32, 5.0f32, 0.0f32, 0.0f32),
                        text: &self.text2,
                    },

                    ScrollBar {
                        Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),
                        orientation: Orientation::Horizontal,
                        value: self.progress.clone(),
                    },
                    ProgressBar {
                        Margin: Thickness::new(5.0f32, 5.0f32, 0.0f32, 0.0f32),
                        value: &self.progress,
                    },

                    DropDown1 {
                        Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),
                        Column: 0,
                        Row: 3,

                        selected_item: self.drop_down_selected_item.clone(),
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
                        text: (&self.drop_down_selected_item, |vm: Option<Rc<StringViewModel>>| match &vm {
                            None => "-".to_string(),
                            Some(vm) => vm.text.clone(),
                        }),
                    },
                },

                BusyIndicator {
                    Column: 0, Row: 0,
                    is_busy: &self.is_busy,

                    Text { text: "Please Wait..." }
                },

                Grid {
                    Column: 0, Row: 1,
                    Margin: Thickness::all(8.0f32),

                    columns: 3,
                    default_height: Length::Auto,

                    Vertical {
                        Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),

                        ToggleButton {
                            Style: Tab {},
                            is_checked: self.is_busy.clone(),
                            Text { text: "Busy Start"}
                        },
                        ToggleButton {
                            Style: Tab {},
                            is_checked: Property::binded_c_two_way(&self.is_busy, |v: bool| { !v }, |v: bool| { !v }),
                            Text { text: "Busy Stop"}
                        },
                    },

                    Vertical {
                        Margin: Thickness::new(5.0f32, 5.0f32, 0.0f32, 0.0f32),
                        radio4,
                        radio5,
                        radio6,
                    },

                    Vertical {
                        Margin: Thickness::new(0.0f32, 5.0f32, 0.0f32, 0.0f32),

                        ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 1"} },
                        ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 2"} },
                        ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 3"} },
                    },
                },
            }
        );

        let tab2 = ui!(
            Grid {
                Title: "Tab 2",

                columns: 2,

                Text {
                    text: (&self.counter, |counter| format!("Counter {}", counter))
                },
                Button {
                    VerticalAlignment: Alignment::Stretch,
                    clicked: Callback::new_vm(&self, |vm, _| vm.decrease()),
                    Text { text: "Decrease" },
                },
                Button {
                    VerticalAlignment: Alignment::Stretch,
                    clicked: Callback::new_vm(&self, |vm, _| vm.increase()),
                    Text { text: "Increase" },
                },
                Text {
                    text: (&self.counter2, |counter| format!("Counter2 {}", counter))
                },
            }
        );

        let exit_callback = Callback::new_async({
            let window = self.window.clone();
            move |_| {
                let window = window.clone();
                async move {
                    if MessageBox::new("Do you really want to exit?")
                        .with_button("Yes")
                        .with_button("No")
                        .show(&window)
                        .await
                        == 0
                    {
                        Application::exit();
                    }
                }
            }
        });

        let input_text_callback = Callback::new_async({
            let window = self.window.clone();
            move |_| {
                let window = window.clone();
                async move {
                    let _ = InputDialog::new("Enter text").get_text(&window, "").await;
                }
            }
        });

        let input_password_callback = Callback::new_async({
            let window = self.window.clone();
            move |_| {
                let window = window.clone();
                async move {
                    let _ = InputDialog::new("Enter password")
                        .get_password(&window)
                        .await;
                }
            }
        });

        let file_open_callback = Callback::new_async({
            let window = self.window.clone();
            move |_| {
                let window = window.clone();
                async move {
                    let file = FileDialog::new()
                        .with_title("Please select a file!")
                        .with_initial_path("/tmp")
                        .with_filter("All files (*.*)", &["*.*"])
                        .with_filter("Markdown (*.md, *.org)", &["*.md", "*.org"])
                        .pick_file()
                        .await;
                    MessageBox::new(format!("{:?}", file))
                        .with_button("Ok")
                        .show(&window)
                        .await;
                }
            }
        });

        let file_save_callback = Callback::new_async({
            let window = self.window.clone();
            move |_| {
                let window = window.clone();
                async move {
                    let file = FileDialog::new()
                        .with_title("Please select a file to save to!")
                        .with_initial_path("test.dat")
                        .with_filter("All files (*.*)", &["*.*"])
                        .with_filter("Markdown (*.md)", &["*.md"])
                        .pick_save_file()
                        .await;
                    MessageBox::new(format!("{:?}", file))
                        .with_button("Ok")
                        .show(&window)
                        .await;
                }
            }
        });

        let menu_items = vec![
            MenuItem::folder(
                "File",
                vec![
                    MenuItem::simple("Open...", file_open_callback),
                    MenuItem::simple("Save...", file_save_callback),
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
                "Dialogs",
                vec![
                    MenuItem::simple("Input text", input_text_callback),
                    MenuItem::simple("Input password", input_password_callback),
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
            data: (radio_controller),
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

#[tokio::main(flavor = "current_thread")]
//#[tokio::main]
async fn main() -> Result<()> {
    LocalSet::new()
        .run_until(async {
            let app = Application::new("Example: layout").await?;

            let mut window = Window::create(
                WindowOptions::new()
                    .with_title("Example: layout")
                    .with_size(800, 600),
            )
            .await?;

            window.set_vm(MainViewModel::new(window.get_window_service()));

            app.run().await?;

            Ok::<(), Error>(())
        })
        .await
}
