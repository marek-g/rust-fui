#![windows_subsystem = "windows"]

use anyhow::Result;
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
use std::thread;
use std::time::Duration;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

struct MainViewModel {
    pub counter: Property<i32>,
    pub counter2: Property<i32>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MainViewModel {
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
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();

        vm.counter2.bind(&mut vm.counter);
        vm.counter.bind(&mut vm.counter2);

        ui!(
            Horizontal {
                Margin: Thickness::sides(0.0f32, 5.0f32),
                Text {
                    Margin: Thickness::all(5.0f32),
                    text: (&vm.counter, |counter| format!("Counter {}", counter))
                },
                Button {
                    clicked: Callback::new(view_model, |vm, _| vm.decrease()),
                    Text { text: "Decrease" }
                },
                ButtonText {
                    clicked: Callback::new(view_model, |vm, _| vm.increase()),
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

fn main() -> Result<()> {
    let mut app = fui_app::Application::new("Example: tray");

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
    tray.set_menu(menu_items);
    tray.set_icon(&icon);
    tray.set_tool_tip("Mądrej Głowie dość po słowie!\nLinia 2\nLinia 3\nLinia 4");
    tray.set_visible(true).unwrap();

    tray.show_message("Title", "Hello world", TrayIconType::Custom(&icon), 5000)
        .unwrap();

    app.add_window(
        WindowOptions::new()
            .with_title("Example: tray")
            .with_size(800, 600),
        MainViewModel::new(),
    )?;

    app.run()?;

    Ok(())
}
