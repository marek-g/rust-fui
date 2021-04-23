use fui_system::*;
use gl::types::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let system_app = Application::new("Example: tray");

    let window_rc = create_new_window();

    let menu_items = vec![
        MenuItem::folder(
            "Window",
            vec![
                MenuItem::simple("Show", {
                    let window_rc_clone = window_rc.clone();
                    move || {
                        window_rc_clone.borrow_mut().set_visible(true);
                    }
                }),
                MenuItem::simple("Hide", {
                    let window_rc_clone = window_rc.clone();
                    move || {
                        window_rc_clone.borrow_mut().set_visible(false);
                    }
                }),
                MenuItem::simple("New", move || {
                    create_new_window();
                }),
            ],
        ),
        MenuItem::Separator,
        MenuItem::simple("Exit", || {
            Application::exit(0);
        }),
    ];

    let mut tray = TrayIcon::new().unwrap();
    let icon_data = std::fs::read("/usr/share/icons/gnome/32x32/actions/add.png").unwrap();
    tray.set_menu(&menu_items);
    tray.set_icon(&icon_data);
    tray.set_tool_tip("Mądrej Głowie dość po słowie!\nLinia 2\nLinia 3\nLinia 4");
    tray.set_visible(true).unwrap();

    //drop(menu_items);

    let icon_data = std::fs::read("/usr/share/icons/gnome/32x32/actions/add.png").unwrap();
    tray.show_message(
        "Title",
        "Hello world",
        TrayIconType::Custom(&icon_data),
        5000,
    )
    .unwrap();

    Application::message_loop();
}

fn create_new_window() -> Rc<RefCell<Window>> {
    let window_rc = Rc::new(RefCell::new(Window::new(None).unwrap()));
    {
        let mut window = window_rc.borrow_mut();
        window.set_title("Hello Qt!").unwrap();
        window.set_visible(true).unwrap();
        window.resize(500, 500);

        window.on_initialize_gl({
            let window_clone = window_rc.clone();
            move || {
                gl::load_with(|s| window_clone.borrow().get_opengl_proc_address(s).unwrap());
            }
        });

        window.on_paint_gl(|| unsafe {
            gl::ClearColor(1.0f32, 0.0f32, 0.0f32, 1.0f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        });
    }
    window_rc
}
