#![windows_subsystem = "windows"]

use fui_system::*;
use fui_system_core::{ElementState, Event, MouseButton, Position, TranslucentEffect};
use rust_embed::RustEmbed;
use std::cell::RefCell;
use std::error::Error;
use std::ptr::null;
use std::rc::Rc;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() -> Result<(), Box<(dyn Error + 'static)>> {
    let app = Application::new(ApplicationOptions::new().with_title("Example: popup")).unwrap();

    let icon_data = Assets::get("icon.png").unwrap();
    let _icon = Icon::from_data(&icon_data.data).unwrap();

    // other windows (keep references to keep windows open)
    let windows = Rc::new(RefCell::new(Vec::new()));

    // first window
    let window_rc = create_new_window(windows);
    window_rc.borrow_mut().set_visible(true).unwrap();

    app.message_loop();

    Ok(())
}

fn create_new_window(windows: Rc<RefCell<Vec<Rc<RefCell<Window>>>>>) -> Rc<RefCell<Window>> {
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
                    Event::MouseMove { position } => {
                        mouse_position.x = position.x;
                        mouse_position.y = position.y
                    }

                    Event::MouseButton {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                    } => {
                        if let Some(window_rc) = window_weak.upgrade() {
                            let popup_window_rc = create_new_popup_window(window_rc.clone());
                            popup_window_rc.borrow_mut().set_visible(true).unwrap();
                            windows.borrow_mut().push(popup_window_rc);
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

fn create_new_popup_window(parent_window_rc: Rc<RefCell<Window>>) -> Rc<RefCell<Window>> {
    //let parent_window_ref = &mut parent_window_rc.borrow_mut();
    //let window_rc = Rc::new(RefCell::new(Window::new(Some(parent_window_ref)).unwrap()));
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
        //window
        //    .set_frame_type(fui_system_core::WindowFrameType::Frameless)
        //    .unwrap();
        //window.set_stay_on_top(true).unwrap();
        //window.set_transparent_for_input(true).unwrap();
        window.set_popup_window();
        window.set_frame_position(800, 100);
        window.resize(200, 200);

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

                gl::ClearColor(0.0f32, 0.5f32, 0.0f32, 0.2f32);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            }
        });

        window.on_event({
            move |event| {
                println!("PopupEvent: {:?}", event);
                false
            }
        });
    }

    window_rc
}
