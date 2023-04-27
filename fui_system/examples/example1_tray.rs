#![windows_subsystem = "windows"]

use fui_system::*;
use fui_system_core::{
    CursorShape, Edge, ElementState, Event, MouseButton, Position, TranslucentEffect,
};
use rust_embed::RustEmbed;
use std::cell::RefCell;
use std::error::Error;
use std::ptr::null;
use std::rc::Rc;
use std::thread;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() -> Result<(), Box<(dyn Error + 'static)>> {
    let app = Application::new(ApplicationOptions::new().with_title("Example: tray")).unwrap();

    let icon_data = Assets::get("icon.png").unwrap();
    let icon = Icon::from_data(&icon_data.data).unwrap();

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
                MenuItem::full(
                    "Show",
                    Some("Ctrl+S".to_string()),
                    Some(Icon::from_data(&icon_data.data).unwrap()),
                    {
                        let window_rc_clone = window_rc.clone();
                        move || {
                            window_rc_clone.borrow_mut().set_visible(true).unwrap();
                        }
                    },
                ),
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
            let icon = Icon::from_data(&icon_data.data).unwrap();
            move || {
                if let Some(tray_rc) = tray_weak.upgrade() {
                    tray_rc
                        .borrow_mut()
                        .show_message("Title", "Hello world", TrayIconType::Custom(&icon), 5000)
                        .unwrap();
                }
            }
        }),
        MenuItem::simple("Post callback", {
            let dispatcher = app.get_dispatcher();
            move || {
                let var = Rc::new(RefCell::new(true));
                dispatcher.post_func_same_thread(move || {
                    println!("Posted function! {}", *var.borrow_mut())
                });
            }
        }),
        MenuItem::Separator,
        MenuItem::simple("Exit", || {
            Application::exit(0);
        }),
    ];

    {
        let mut tray = tray_rc.borrow_mut();
        tray.set_menu(menu_items).unwrap();
        tray.set_icon(&icon).unwrap();
        tray.set_tool_tip("Mądrej Głowie dość po słowie!\nLinia 2\nLinia 3\nLinia 4")
            .unwrap();
        tray.set_visible(true).unwrap();
    }

    let thread_handler = thread::spawn(move || {
        Application::post_func(move || println!("Function posted from another thread!"));
    });

    app.message_loop();

    thread_handler.join().unwrap();

    Ok(())
}

fn create_new_window() -> Rc<RefCell<Window>> {
    let window_rc = Rc::new(RefCell::new(Window::new(None).unwrap()));
    {
        let icon_data = Assets::get("icon.png").unwrap();
        let icon = Icon::from_data(&icon_data.data).unwrap();

        let mut window = window_rc.borrow_mut();
        window.set_title("Hello Qt!").unwrap();
        window.set_icon(&icon).unwrap();
        window
            .set_translucent_background(TranslucentEffect::Blur)
            .unwrap();
        //window.set_frame_type(WindowFrameType::Frameless).unwrap();
        //window.set_stay_on_top(true).unwrap();
        //window.set_transparent_for_input(true).unwrap();
        window.resize(500, 500);

        let mut initialized = false;
        window.on_paint_gl({
            let window_weak = Rc::downgrade(&window_rc);
            move || unsafe {
                if !initialized {
                    if let Some(window_rc) = window_weak.upgrade() {
                        gl::load_with(|s| {
                            window_rc
                                .borrow()
                                .get_opengl_proc_address(s)
                                .unwrap_or_else(|_| null())
                        });
                    }
                    initialized = true;
                }

                gl::ClearColor(0.5f32, 0.0f32, 0.0f32, 0.2f32);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            }
        });

        window.on_event({
            let window_weak = Rc::downgrade(&window_rc);
            let mut mouse_position = Box::new(Position {
                x: 0.0f32,
                y: 0.0f32,
            });
            move |event| {
                println!("Event: {:?}", event);

                match event {
                    Event::MouseButton {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                    } => {
                        if let Some(window_rc) = window_weak.upgrade() {
                            let width = window_rc.borrow_mut().get_width() as f32;
                            let height = window_rc.borrow_mut().get_height() as f32;

                            let edge =
                                position_to_edge(mouse_position.x, mouse_position.y, width, height);
                            if !edge.is_empty() {
                                window_rc.borrow_mut().start_system_resize(edge);
                            } else {
                                window_rc.borrow_mut().start_system_move();
                            }
                        }
                    }

                    Event::MouseMove { position } => {
                        mouse_position.x = position.x;
                        mouse_position.y = position.y;

                        if let Some(window_rc) = window_weak.upgrade() {
                            let width = window_rc.borrow_mut().get_width() as f32;
                            let height = window_rc.borrow_mut().get_height() as f32;

                            let edge =
                                position_to_edge(mouse_position.x, mouse_position.y, width, height);

                            let cursor = match edge {
                                Edge::Left | Edge::Right => CursorShape::SizeHorCursor,
                                Edge::Top | Edge::Bottom => CursorShape::SizeVerCursor,
                                Edge::TopLeft | Edge::BottomRight => CursorShape::SizeFDiagCursor,
                                Edge::TopRight | Edge::BottomLeft => CursorShape::SizeBDiagCursor,
                                _ => CursorShape::CrossCursor,
                            };

                            window_rc.borrow_mut().set_cursor(cursor);
                        }
                    }

                    _ => (),
                }

                false
            }
        });
    }

    window_rc
}

fn position_to_edge(x: f32, y: f32, width: f32, height: f32) -> Edge {
    let mut edge = 0i32;

    if x >= 0.0f32 && x <= 10.0f32 {
        edge += Edge::Left.bits();
    } else if x >= width - 11.0f32 && x < width {
        edge += Edge::Right.bits();
    }

    if y >= 0.0f32 && y <= 10.0f32 {
        edge += Edge::Top.bits();
    } else if y >= height - 11.0f32 && y < height {
        edge += Edge::Bottom.bits();
    }

    Edge::from_bits(edge).unwrap()
}
