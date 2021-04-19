use fui_system::*;

fn main() {
    let system_app = SystemApplication::new("Example: tray");

    let menu_items = vec![
        SystemMenuItem::folder(
            "File",
            vec![
                SystemMenuItem::simple("Open...", None),
                SystemMenuItem::simple("Save...", None),
                SystemMenuItem::folder(
                    "Export",
                    vec![
                        SystemMenuItem::simple("PDF...", None),
                        SystemMenuItem::simple("PNG...", None),
                        SystemMenuItem::simple("HTML...", None),
                    ],
                ),
                SystemMenuItem::Separator,
                SystemMenuItem::simple("Exit", None),
            ],
        ),
        SystemMenuItem::folder(
            "Help",
            vec![
                SystemMenuItem::simple("Help", None),
                SystemMenuItem::Separator,
                SystemMenuItem::simple("About", None),
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

    let mut window = SystemWindow::new(None).unwrap();
    window.set_title("Hello Qt!").unwrap();
    window.set_visible(true).unwrap();

    window.set_initialize_gl_callback(|| println!("InitializeGL callback."));
    window.set_paint_gl_callback(|| println!("PaintGL callback."));

    SystemApplication::message_loop();
}
