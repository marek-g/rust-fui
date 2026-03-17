#![windows_subsystem = "windows"]

use anyhow::Result;
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::Rc;
use tokio::task::LocalSet;

struct MainViewModel {
    // Services
    window_service: Rc<dyn WindowService>,
    file_service: Rc<dyn FileDialogService>,

    // Properties
    pub text: Property<String>,
    pub text2: Property<String>,
    pub progress: Property<f32>,
    pub is_busy: Property<bool>,
    pub counter: Property<i32>,
    pub counter2: Property<i32>,
    pub drop_down_selected_item: Property<Option<Rc<StringViewModel>>>,
}

impl MainViewModel {
    pub fn new(services: Services) -> Rc<Self> {
        Rc::new(MainViewModel {
            window_service: services
                .get_window_service()
                .clone()
                .expect("WindowService is missing"),
            file_service: services.get_file_dialog_service(),

            text: "My text".into(),
            text2: "ąęść".into(),
            progress: 0.5.into(),
            is_busy: false.into(),
            counter: 10.into(),
            counter2: 0.into(),
            drop_down_selected_item: Property::new(None),
        })
    }

    pub fn increase(self: &Rc<Self>) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(self: &Rc<Self>) {
        self.counter.change(|c| c - 1);
    }

    pub async fn file_open(self: &Rc<Self>) {
        let file = self
            .file_service
            .pick_file(
                FileDialogData::new()
                    .with_title("Please select a file!")
                    .with_initial_path("/tmp")
                    .with_filter("All files (*.*)", &["*.*"])
                    .with_filter("Markdown (*.md, *.org)", &["*.md", "*.org"]),
            )
            .await;

        MessageBox::new(format!("{:?}", file))
            .with_button("Ok")
            .show(&self.window_service)
            .await;
    }

    pub async fn file_save(self: &Rc<Self>) {
        let file = self
            .file_service
            .pick_save_file(
                FileDialogData::new()
                    .with_title("Please select a file to save to!")
                    .with_initial_path("test.dat")
                    .with_filter("All files (*.*)", &["*.*"])
                    .with_filter("Markdown (*.md)", &["*.md"]),
            )
            .await;

        MessageBox::new(format!("{:?}", file))
            .with_button("Ok")
            .show(&self.window_service)
            .await;
    }

    pub async fn input_text(self: &Rc<Self>) {
        let _ = InputDialog::new("Enter text")
            .get_text(&self.window_service, "")
            .await;
    }

    pub async fn input_password(self: &Rc<Self>) {
        let _ = InputDialog::new("Enter password")
            .get_password(&self.window_service)
            .await;
    }

    pub async fn exit_app(self: &Rc<Self>) {
        if MessageBox::new("Do you really want to exit?")
            .with_button("Yes")
            .with_button("No")
            .show(&self.window_service)
            .await
            == 0
        {
            Application::exit();
        }
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
                    Margin: Thickness::all(8.0),
                    columns: 2,
                    default_height: Length::Auto,

                    TextBox { text: self.text.clone() },
                    Text { Margin: Thickness::left(5.0), text: &self.text },

                    TextBox {
                        Style: Default { password: true },
                        Margin: Thickness::new(0.0, 5.0, 0.0, 0.0),
                        text: self.text2.clone(),
                    },
                    Text {
                        Style: Default { color: [1.0, 0.8, 0.0, 1.0] },
                        Margin: Thickness::new(5.0, 5.0, 0.0, 0.0),
                        text: &self.text2,
                    },

                    ScrollBar {
                        Margin: Thickness::new(0.0, 5.0, 0.0, 0.0),
                        orientation: Orientation::Horizontal,
                        value: self.progress.clone(),
                    },
                    ProgressBar {
                        Margin: Thickness::new(5.0, 5.0, 0.0, 0.0),
                        value: &self.progress,
                    },

                    DropDown1 {
                        Margin: Thickness::new(0.0, 5.0, 0.0, 0.0),
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
                        Margin: Thickness::new(5.0, 5.0, 0.0, 0.0),
                        text: format!("{}", self.drop_down_selected_item.get().map_or("-".to_string(), |vm| vm.text.clone()))
                    },
                },

                BusyIndicator {
                    Column: 0, Row: 0,
                    is_busy: &self.is_busy,

                    Text { text: "Please Wait..." }
                },

                Grid {
                    Column: 0, Row: 1,
                    Margin: Thickness::all(8.0),
                    columns: 3,
                    default_height: Length::Auto,

                    Vertical {
                        Margin: Thickness::new(0.0, 5.0, 0.0, 0.0),

                        ToggleButton {
                            Style: Tab {},
                            is_checked: self.is_busy.clone(),
                            Text { text: "Busy Start" }
                        },
                        ToggleButton {
                            Style: Tab {},
                            is_checked: Property::binded_c_two_way(&self.is_busy, |v: bool| { !v }, |v: bool| { !v }),
                            Text { text: "Busy Stop" }
                        },
                    },

                    Vertical {
                        Margin: Thickness::new(5.0, 5.0, 0.0, 0.0),
                        radio4, radio5, radio6,
                    },

                    Vertical {
                        Margin: Thickness::new(0.0, 5.0, 0.0, 0.0),

                        ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 1"} },
                        ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 2"} },
                        ToggleButton { Style: CheckBox {}, Text { text: "CheckBox 3"} },
                    },
                },

                PathEdit {
                    Column: 0, Row: 2,
                    label: "Save file: ",
                    kind: PathKind::OpenFile,
                    filters: vec![
                        FileFilter { name: "All files (*.*)".to_string(), filters: vec!["*".to_string()] },
                        FileFilter { name: "Movies (*.mpg, *.avi)".to_string(), filters: vec!["mpg".to_string(), "avi".to_string()] },
                        FileFilter { name: "Markdown (*.md)".to_string(), filters: vec!["md".to_string()]}
                    ],
                },
            }
        );

        let tab2 = ui!(
            Grid {
                Title: "Tab 2",
                columns: 2,
                Text { text: format!("Counter {}", self.counter.get()) },
                Button {
                    VerticalAlignment: Alignment::Stretch,
                    clicked => self.decrease(),
                    Text { text: "Decrease" },
                },
                Button {
                    VerticalAlignment: Alignment::Stretch,
                    clicked => self.increase(),
                    Text { text: "Increase" },
                },
                Text { text: format!("Counter2 {}", self.counter2.get()) },
            }
        );

        let content = ui!(Grid {
            rows: 2,
            heights: vec![(0, Length::Auto)],

            MenuBar {
                Menu {
                    Text { Style: Default { color: [0.0, 0.0, 0.0, 1.0] }, text: "File" },

                    MenuItem {
                        activated async => self.file_open(),
                        Text { Style: Default { color: [0.0, 0.0, 0.0, 1.0] }, text: "Open..."  }
                    },

                    MenuItem {
                        activated async => self.file_save(),
                        Text { text: "Save..." }
                    },

                    SubMenu {
                        Text { text: "Export" },

                        MenuItem {
                            activated => {},
                            Text { text: "PDF..." }
                        },

                        MenuItem {
                            activated => {},
                            Text { text: "PNG..." }
                        },

                        MenuItem {
                            activated => {},
                            Text { text: "HTML..." }
                        },
                    },

                    MenuSeparator {},

                    MenuItem {
                        activated async => self.exit_app(),
                        Text { text: "Exit" }
                    },
                },

                Menu {
                    Text { text: "Dialogs" },

                    MenuItem {
                        activated async => self.input_text(),
                        Text { text: "Input text" }
                    },

                    MenuItem {
                        activated async => self.input_password(),
                        Text { text: "Input password" }
                    },
                },

                Menu {
                    Text { text: "Help" },

                    MenuItem {
                        activated => {},
                        Text { text: "Help" }
                    },

                    MenuSeparator {},

                    MenuItem {
                        activated => {},
                        Text { text: "About" }
                    },
                },
            },

            TabControl {
                Margin: Thickness::new(8.0, 12.0, 8.0, 8.0),

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
                inherited_values: InheritedTypeMap::new(),
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

            window.set_vm(MainViewModel::new(window.get_services()));

            app.run().await?;

            Ok(())
        })
        .await
}
