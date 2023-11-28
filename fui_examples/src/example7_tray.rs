#![windows_subsystem = "windows"]

use anyhow::{Error, Result};
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;
use fui_system::*;

use std::cell::RefCell;
use std::rc::Rc;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use tokio::task::LocalSet;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

struct MainViewModel {
    pub counter: Property<i32>,
    pub counter2: Property<i32>,
}

impl MainViewModel {
    pub fn new() -> Rc<Self> {
        Rc::new(MainViewModel {
            counter: Property::new(10),
            counter2: Property::new(0),
        })
    }

    pub fn increase(&self) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(&self) {
        self.counter.change(|c| c - 1);
    }
}

#[derive(TypedBuilder)]
pub struct ButtonText {
    #[builder(default = Property::new("".to_string()))]
    pub text: Property<String>,
    #[builder(default = Callback::empty())]
    pub clicked: Callback<()>,
}

impl ButtonText {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            Button {
                clicked: self.clicked,
                Text { text: self.text }
            }
        }
    }
}

impl ViewModel for MainViewModel {
    fn create_view(vm: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>> {
        vm.counter2.bind(&vm.counter);
        vm.counter.bind(&vm.counter2);

        ui!(
            Horizontal {
                Margin: Thickness::sides(0.0f32, 5.0f32),
                Text {
                    Margin: Thickness::all(5.0f32),
                    text: (&vm.counter, |counter| format!("Counter {}", counter))
                },
                Button {
                    clicked: Callback::new_vm(vm, |vm, _| vm.decrease()),
                    Text { text: "Decrease" }
                },
                ButtonText {
                    clicked: Callback::new_vm(vm, |vm, _| vm.increase()),
                    text: "Increase"
                },
                Text {
                    Margin: Thickness::all(5.0f32),
                    text: (&vm.counter2, |counter| format!("Counter2 {}", counter))
                },
            }
        )
    }
}

#[tokio::main(flavor = "current_thread")]
//#[tokio::main]
async fn main() -> Result<()> {
    LocalSet::new()
        .run_until(async {
            // TODO: tray must be still ported to async
            let app = fui_app::Application::new("Example: tray").await?;

            let menu_items = vec![
                fui_system::MenuItem::folder(
                    "File",
                    vec![
                        fui_system::MenuItem::simple("Open...", || {}),
                        fui_system::MenuItem::simple("Save...", || {}),
                        fui_system::MenuItem::folder(
                            "Export",
                            vec![
                                fui_system::MenuItem::simple("PDF...", || {}),
                                fui_system::MenuItem::simple("PNG...", || {}),
                                fui_system::MenuItem::simple("HTML...", || {}),
                            ],
                        ),
                        fui_system::MenuItem::Separator,
                        fui_system::MenuItem::simple("Exit", || fui_app::Application::exit()),
                    ],
                ),
                fui_system::MenuItem::folder(
                    "Help",
                    vec![
                        fui_system::MenuItem::simple("Help", || {}),
                        fui_system::MenuItem::Separator,
                        fui_system::MenuItem::simple("About", || {}),
                    ],
                ),
            ];

            let icon_path = Path::new("assets")
                .join("icon.png")
                .into_os_string()
                .into_string()
                .unwrap();
            let mut file = File::open(icon_path).unwrap();
            let mut icon_data = Vec::new();
            file.read_to_end(&mut icon_data)?;
            let icon = Icon::from_data(&icon_data).unwrap();

            let mut tray = TrayIcon::new().unwrap();
            tray.set_menu(menu_items).unwrap();
            tray.set_icon(&icon).unwrap();
            tray.set_tool_tip("Mądrej Głowie dość po słowie!\nLinia 2\nLinia 3\nLinia 4")
                .unwrap();
            tray.set_visible(true).unwrap();

            tray.show_message("Title", "Hello world", TrayIconType::Custom(&icon), 5000)
                .unwrap();

            let mut window = fui_app::Window::create(
                WindowOptions::new()
                    .with_title("Example: tray")
                    .with_size(800, 600),
            )
            .await?;

            window.set_vm(MainViewModel::new());

            app.run().await?;

            Ok::<(), Error>(())
        })
        .await
}
