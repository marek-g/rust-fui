use fui_system::*;
use gl::types::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let system_app = SystemApplication::new("Example: tray");

    let menu_items = vec![
        SystemMenuItem::folder(
            "File",
            vec![
                SystemMenuItem::simple("Open...", || {}),
                SystemMenuItem::simple("Save...", || {}),
                SystemMenuItem::folder(
                    "Export",
                    vec![
                        SystemMenuItem::simple("PDF...", || {}),
                        SystemMenuItem::simple("PNG...", || {}),
                        SystemMenuItem::simple("HTML...", || {}),
                    ],
                ),
                SystemMenuItem::Separator,
                SystemMenuItem::simple("Exit", || {}),
            ],
        ),
        SystemMenuItem::folder(
            "Help",
            vec![
                SystemMenuItem::simple("Help", || {}),
                SystemMenuItem::Separator,
                SystemMenuItem::simple("About", || {}),
            ],
        ),
        SystemMenuItem::simple("Exit", || {
            SystemApplication::exit_message_loop();
        }),
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

    //drop(menu_items);

    let icon_data = std::fs::read("/usr/share/icons/gnome/32x32/actions/add.png").unwrap();
    tray.show_message(
        "Title",
        "Hello world",
        SystemMessageIcon::Custom(&icon_data),
        5000,
    )
    .unwrap();

    let window_rc = Rc::new(RefCell::new(SystemWindow::new(None).unwrap()));
    {
        let mut window = window_rc.borrow_mut();
        window.set_title("Hello Qt!").unwrap();
        window.set_visible(true).unwrap();

        let window_clone = window_rc.clone();
        window.set_initialize_gl_callback(move || {
            gl::load_with(|s| window_clone.borrow().get_opengl_proc_address(s).unwrap());
        });

        window.set_paint_gl_callback(|| unsafe {
            gl::ClearColor(1.0f32, 0.0f32, 0.0f32, 1.0f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        });
    }

    SystemApplication::message_loop();
}
