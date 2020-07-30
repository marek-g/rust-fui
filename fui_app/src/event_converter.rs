pub fn convert_event(event: &winit::event::WindowEvent) -> Option<fui_core::InputEvent> {
    match event {
        winit::event::WindowEvent::CursorEntered { .. } => {
            Some(fui_core::InputEvent::CursorEntered {})
        }

        winit::event::WindowEvent::CursorLeft { .. } => Some(fui_core::InputEvent::CursorLeft {}),

        winit::event::WindowEvent::CursorMoved { position, .. } => {
            Some(fui_core::InputEvent::CursorMoved {
                position: fui_core::Point::new(position.x as f32, position.y as f32),
            })
        }

        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            Some(fui_core::InputEvent::MouseInput {
                state: match state {
                    winit::event::ElementState::Pressed => fui_core::ElementState::Pressed,
                    winit::event::ElementState::Released => fui_core::ElementState::Released,
                },

                button: match button {
                    winit::event::MouseButton::Left => fui_core::MouseButton::Left,
                    winit::event::MouseButton::Right => fui_core::MouseButton::Right,
                    winit::event::MouseButton::Middle => fui_core::MouseButton::Middle,
                    winit::event::MouseButton::Other(other) => fui_core::MouseButton::Other(*other),
                },
            })
        }

        winit::event::WindowEvent::KeyboardInput { input, .. } => {
            Some(fui_core::InputEvent::KeyboardInput(fui_core::KeyEvent {
                state: match input.state {
                    winit::event::ElementState::Pressed => fui_core::KeyState::Pressed,
                    winit::event::ElementState::Released => fui_core::KeyState::Released,
                },

                keycode: match input.virtual_keycode {
                    Some(winit::event::VirtualKeyCode::Escape) => Some(fui_core::Keycode::Esc),
                    Some(winit::event::VirtualKeyCode::F1) => Some(fui_core::Keycode::F1),
                    Some(winit::event::VirtualKeyCode::F2) => Some(fui_core::Keycode::F2),
                    Some(winit::event::VirtualKeyCode::F3) => Some(fui_core::Keycode::F3),
                    Some(winit::event::VirtualKeyCode::F4) => Some(fui_core::Keycode::F4),
                    Some(winit::event::VirtualKeyCode::F5) => Some(fui_core::Keycode::F5),
                    Some(winit::event::VirtualKeyCode::F6) => Some(fui_core::Keycode::F6),
                    Some(winit::event::VirtualKeyCode::F7) => Some(fui_core::Keycode::F7),
                    Some(winit::event::VirtualKeyCode::F8) => Some(fui_core::Keycode::F8),
                    Some(winit::event::VirtualKeyCode::F9) => Some(fui_core::Keycode::F9),
                    Some(winit::event::VirtualKeyCode::F10) => Some(fui_core::Keycode::F10),
                    Some(winit::event::VirtualKeyCode::F11) => Some(fui_core::Keycode::F11),
                    Some(winit::event::VirtualKeyCode::F12) => Some(fui_core::Keycode::F12),
                    Some(winit::event::VirtualKeyCode::Snapshot) => {
                        Some(fui_core::Keycode::PrintScreen)
                    }
                    Some(winit::event::VirtualKeyCode::Scroll) => {
                        Some(fui_core::Keycode::ScrollLock)
                    }
                    Some(winit::event::VirtualKeyCode::Pause) => Some(fui_core::Keycode::Pause),
                    Some(winit::event::VirtualKeyCode::Insert) => Some(fui_core::Keycode::Insert),
                    Some(winit::event::VirtualKeyCode::Home) => Some(fui_core::Keycode::Home),
                    Some(winit::event::VirtualKeyCode::Delete) => Some(fui_core::Keycode::Delete),
                    Some(winit::event::VirtualKeyCode::End) => Some(fui_core::Keycode::End),
                    Some(winit::event::VirtualKeyCode::PageDown) => {
                        Some(fui_core::Keycode::PageDown)
                    }
                    Some(winit::event::VirtualKeyCode::PageUp) => Some(fui_core::Keycode::PageUp),
                    Some(winit::event::VirtualKeyCode::Left) => Some(fui_core::Keycode::Left),
                    Some(winit::event::VirtualKeyCode::Right) => Some(fui_core::Keycode::Right),
                    Some(winit::event::VirtualKeyCode::Up) => Some(fui_core::Keycode::Up),
                    Some(winit::event::VirtualKeyCode::Down) => Some(fui_core::Keycode::Down),
                    Some(winit::event::VirtualKeyCode::Back) => Some(fui_core::Keycode::Backspace),
                    Some(winit::event::VirtualKeyCode::Tab) => Some(fui_core::Keycode::Tab),
                    Some(winit::event::VirtualKeyCode::Return) => Some(fui_core::Keycode::Enter),
                    Some(winit::event::VirtualKeyCode::LShift) => Some(fui_core::Keycode::LShift),
                    Some(winit::event::VirtualKeyCode::RShift) => Some(fui_core::Keycode::RShift),
                    Some(winit::event::VirtualKeyCode::LControl) => Some(fui_core::Keycode::LCtrl),
                    Some(winit::event::VirtualKeyCode::RControl) => Some(fui_core::Keycode::RCtrl),
                    Some(winit::event::VirtualKeyCode::LAlt) => Some(fui_core::Keycode::LAlt),
                    Some(winit::event::VirtualKeyCode::RAlt) => Some(fui_core::Keycode::RAlt),
                    Some(winit::event::VirtualKeyCode::LWin) => Some(fui_core::Keycode::LWin),
                    Some(winit::event::VirtualKeyCode::RWin) => Some(fui_core::Keycode::RWin),
                    Some(winit::event::VirtualKeyCode::Numlock) => Some(fui_core::Keycode::NumLock),
                    Some(winit::event::VirtualKeyCode::NumpadEnter) => {
                        Some(fui_core::Keycode::NumpadEnter)
                    }
                    Some(winit::event::VirtualKeyCode::Copy) => Some(fui_core::Keycode::Copy),
                    Some(winit::event::VirtualKeyCode::Paste) => Some(fui_core::Keycode::Paste),
                    Some(winit::event::VirtualKeyCode::Cut) => Some(fui_core::Keycode::Cut),
                    _ => None,
                },

                text: None,

                modifiers: fui_core::KeyModifiers {
                    shift: input.modifiers.shift(),
                    ctrl: input.modifiers.ctrl(),
                    alt: input.modifiers.alt(),
                    win: input.modifiers.logo(),
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

            Some(fui_core::InputEvent::KeyboardInput(fui_core::KeyEvent {
                state: fui_core::KeyState::Pressed,
                keycode: None,
                text: Some(character.to_string()),
                modifiers: fui_core::KeyModifiers {
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
