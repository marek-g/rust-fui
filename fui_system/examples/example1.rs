#![windows_subsystem = "windows"]

use fui_system::*;
use rust_embed::RustEmbed;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() {
    let _system_app = Application::new(
        ApplicationOptionsBuilder::new()
            .with_title("Example: tray")
            .build(),
    );

    let icon_data = Assets::get("icon.png").unwrap();
    let icon = Icon::from_data(&icon_data).unwrap();

    // first window
    let window_rc = create_new_window();
    window_rc.borrow_mut().set_visible(true).unwrap();

    // other windows (keep references to keep windows open)
    let mut windows = Vec::new();

    // tray icon
    let tray_rc = Rc::new(RefCell::new(TrayIcon::new().unwrap()));

    // menu
    let menu_items = vec![
        MenuItem::folder(
            "Window",
            vec![
                MenuItem::simple("Show", {
                    let window_rc_clone = window_rc.clone();
                    move || {
                        window_rc_clone.borrow_mut().set_visible(true).unwrap();
                    }
                }),
                MenuItem::simple("Hide", {
                    let window_rc_clone = window_rc.clone();
                    move || {
                        window_rc_clone.borrow_mut().set_visible(false).unwrap();
                    }
                }),
                MenuItem::simple("New", move || {
                    let window_rc = create_new_window();
                    window_rc.borrow_mut().set_visible(true).unwrap();
                    windows.push(window_rc);
                }),
            ],
        ),
        MenuItem::Separator,
        MenuItem::simple("Show tray message", {
            let tray_weak = Rc::downgrade(&tray_rc);
            let icon_data = Assets::get("icon.png").unwrap();
            let icon = Icon::from_data(&icon_data).unwrap();
            move || {
                if let Some(tray_rc) = tray_weak.upgrade() {
                    tray_rc
                        .borrow_mut()
                        .show_message("Title", "Hello world", TrayIconType::Custom(&icon), 5000)
                        .unwrap();
                }
            }
        }),
        MenuItem::Separator,
        MenuItem::simple("Exit", || {
            Application::exit(0);
        }),
    ];

    {
        let mut tray = tray_rc.borrow_mut();
        tray.set_menu(&menu_items).unwrap();
        tray.set_icon(&icon).unwrap();
        tray.set_tool_tip("Mądrej Głowie dość po słowie!\nLinia 2\nLinia 3\nLinia 4")
            .unwrap();
        tray.set_visible(true).unwrap();
    }

    //drop(menu_items);

    Application::message_loop();
}

fn create_new_window() -> Rc<RefCell<Window>> {
    let window_rc = Rc::new(RefCell::new(Window::new(None).unwrap()));
    {
        let icon_data = Assets::get("icon.png").unwrap();
        let icon = Icon::from_data(&icon_data).unwrap();

        let mut window = window_rc.borrow_mut();
        window.set_title("Hello Qt!").unwrap();
        window.set_icon(&icon).unwrap();
        window.resize(500, 500);

        let mut initialized = false;
        let window_weak = Rc::downgrade(&window_rc);
        window.on_paint_gl(move || unsafe {
            if !initialized {
                if let Some(window_rc) = window_weak.upgrade() {
                    gl::load_with(|s| window_rc.borrow().get_opengl_proc_address(s).unwrap());
                }
                initialized = true;
            }

            gl::ClearColor(1.0f32, 0.0f32, 0.0f32, 0.5f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        });
    }
    window_rc
}
