pub fn convert_event(event: &winit::event::WindowEvent) -> Option<fui::InputEvent> {
    match event {
        winit::event::WindowEvent::CursorEntered { .. } => Some(fui::InputEvent::CursorEntered {}),

        winit::event::WindowEvent::CursorLeft { .. } => Some(fui::InputEvent::CursorLeft {}),

        winit::event::WindowEvent::CursorMoved { position, .. } => {
            Some(fui::InputEvent::CursorMoved {
                position: fui::Point::new(position.x as f32, position.y as f32),
            })
        }

        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            Some(fui::InputEvent::MouseInput {
                state: match state {
                    winit::event::ElementState::Pressed => fui::ElementState::Pressed,
                    winit::event::ElementState::Released => fui::ElementState::Released,
                },

                button: match button {
                    winit::event::MouseButton::Left => fui::MouseButton::Left,
                    winit::event::MouseButton::Right => fui::MouseButton::Right,
                    winit::event::MouseButton::Middle => fui::MouseButton::Middle,
                    winit::event::MouseButton::Other(other) => fui::MouseButton::Other(*other),
                },
            })
        }

        winit::event::WindowEvent::KeyboardInput { input, .. } => {
            Some(fui::InputEvent::KeyboardInput(fui::KeyEvent {
                state: match input.state {
                    winit::event::ElementState::Pressed => fui::KeyState::Pressed,
                    winit::event::ElementState::Released => fui::KeyState::Released,
                },

                keycode: match input.virtual_keycode {
                    Some(winit::event::VirtualKeyCode::Escape) => Some(fui::Keycode::Esc),
                    Some(winit::event::VirtualKeyCode::F1) => Some(fui::Keycode::F1),
                    Some(winit::event::VirtualKeyCode::F2) => Some(fui::Keycode::F2),
                    Some(winit::event::VirtualKeyCode::F3) => Some(fui::Keycode::F3),
                    Some(winit::event::VirtualKeyCode::F4) => Some(fui::Keycode::F4),
                    Some(winit::event::VirtualKeyCode::F5) => Some(fui::Keycode::F5),
                    Some(winit::event::VirtualKeyCode::F6) => Some(fui::Keycode::F6),
                    Some(winit::event::VirtualKeyCode::F7) => Some(fui::Keycode::F7),
                    Some(winit::event::VirtualKeyCode::F8) => Some(fui::Keycode::F8),
                    Some(winit::event::VirtualKeyCode::F9) => Some(fui::Keycode::F9),
                    Some(winit::event::VirtualKeyCode::F10) => Some(fui::Keycode::F10),
                    Some(winit::event::VirtualKeyCode::F11) => Some(fui::Keycode::F11),
                    Some(winit::event::VirtualKeyCode::F12) => Some(fui::Keycode::F12),
                    Some(winit::event::VirtualKeyCode::Snapshot) => Some(fui::Keycode::PrintScreen),
                    Some(winit::event::VirtualKeyCode::Scroll) => Some(fui::Keycode::ScrollLock),
                    Some(winit::event::VirtualKeyCode::Pause) => Some(fui::Keycode::Pause),
                    Some(winit::event::VirtualKeyCode::Insert) => Some(fui::Keycode::Insert),
                    Some(winit::event::VirtualKeyCode::Home) => Some(fui::Keycode::Home),
                    Some(winit::event::VirtualKeyCode::Delete) => Some(fui::Keycode::Delete),
                    Some(winit::event::VirtualKeyCode::End) => Some(fui::Keycode::End),
                    Some(winit::event::VirtualKeyCode::PageDown) => Some(fui::Keycode::PageDown),
                    Some(winit::event::VirtualKeyCode::PageUp) => Some(fui::Keycode::PageUp),
                    Some(winit::event::VirtualKeyCode::Left) => Some(fui::Keycode::Left),
                    Some(winit::event::VirtualKeyCode::Right) => Some(fui::Keycode::Right),
                    Some(winit::event::VirtualKeyCode::Up) => Some(fui::Keycode::Up),
                    Some(winit::event::VirtualKeyCode::Down) => Some(fui::Keycode::Down),
                    Some(winit::event::VirtualKeyCode::Back) => Some(fui::Keycode::Backspace),
                    Some(winit::event::VirtualKeyCode::Tab) => Some(fui::Keycode::Tab),
                    Some(winit::event::VirtualKeyCode::Return) => Some(fui::Keycode::Enter),
                    Some(winit::event::VirtualKeyCode::LShift) => Some(fui::Keycode::LShift),
                    Some(winit::event::VirtualKeyCode::RShift) => Some(fui::Keycode::RShift),
                    Some(winit::event::VirtualKeyCode::LControl) => Some(fui::Keycode::LCtrl),
                    Some(winit::event::VirtualKeyCode::RControl) => Some(fui::Keycode::RCtrl),
                    Some(winit::event::VirtualKeyCode::LAlt) => Some(fui::Keycode::LAlt),
                    Some(winit::event::VirtualKeyCode::RAlt) => Some(fui::Keycode::RAlt),
                    Some(winit::event::VirtualKeyCode::LWin) => Some(fui::Keycode::LWin),
                    Some(winit::event::VirtualKeyCode::RWin) => Some(fui::Keycode::RWin),
                    Some(winit::event::VirtualKeyCode::Numlock) => Some(fui::Keycode::NumLock),
                    Some(winit::event::VirtualKeyCode::NumpadEnter) => {
                        Some(fui::Keycode::NumpadEnter)
                    }
                    Some(winit::event::VirtualKeyCode::Copy) => Some(fui::Keycode::Copy),
                    Some(winit::event::VirtualKeyCode::Paste) => Some(fui::Keycode::Paste),
                    Some(winit::event::VirtualKeyCode::Cut) => Some(fui::Keycode::Cut),
                    _ => None,
                },

                text: None,

                modifiers: fui::KeyModifiers {
                    shift: input.modifiers.shift,
                    ctrl: input.modifiers.ctrl,
                    alt: input.modifiers.alt,
                    win: input.modifiers.logo,
                },
            }))
        }

        winit::event::WindowEvent::ReceivedCharacter(character) => {
            // filter out characters matching existing keycodes
            match character {
                // backspace
                '\x08' |
                // delete
                '\x7F' |
                // escape
                '\x1B' |
                // tab
                '\t' |
                // enter
                '\r' => return None,
                _ => (),
            }

            Some(fui::InputEvent::KeyboardInput(fui::KeyEvent {
                state: fui::KeyState::Pressed,
                keycode: None,
                text: Some(character.to_string()),
                modifiers: fui::KeyModifiers {
                    shift: false,
                    ctrl: false,
                    alt: false,
                    win: false,
                },
            }))
        }

        _ => None,
    }
}
