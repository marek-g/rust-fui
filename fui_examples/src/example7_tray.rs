#![windows_subsystem = "windows"]

use anyhow::Result;
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;
use fui_system::*;

use std::cell::RefCell;
use std::rc::Rc;

use std::thread;
use std::time::Duration;
use typed_builder::TypedBuilder;
use typemap::TypeMap;
use winit::window::WindowBuilder;

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
    let mut app = Application::new("Example: tray").unwrap();

    let tray_thread = thread::spawn(move || {
        let system_app = SystemApplication::new("Example: tray");

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
                    MenuItem::simple("Exit", Callback::empty()),
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

        let mut tray = SystemTray::new().unwrap();
        let icon_data = std::fs::read("/usr/share/icons/gnome/32x32/actions/add.png").unwrap();
        tray.set_menu(&menu_items);
        tray.set_icon(&icon_data);
        tray.set_tool_tip("Mądrej Głowie dość po słowie!\nLinia 2\nLinia 3\nLinia 4");
        tray.set_visible(true).unwrap();

        let mut tray2 = SystemTray::new().unwrap();
        tray2.set_menu(&menu_items);
        tray2.set_icon(&icon_data);
        tray2.set_visible(true).unwrap();

        let icon_data = std::fs::read("/usr/share/icons/gnome/32x32/actions/add.png").unwrap();
        tray.show_message(
            "Title",
            "Hello world",
            SystemMessageIcon::Custom(&icon_data),
            5000,
        )
        .unwrap();

        SystemApplication::message_loop();
    });

    //std::thread::sleep(Duration::from_secs(2));
    //SystemApplication::exit_message_loop();

    app.add_window(
        WindowBuilder::new().with_title("Example: tray"),
        MainViewModel::new(),
    )?;

    app.run();

    Ok(())
}
