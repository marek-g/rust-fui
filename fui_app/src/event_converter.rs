use fui_system_core::{Event, ScrollDelta};

pub fn convert_event(event: &Event) -> Option<fui_core::InputEvent> {
    match event {
        Event::MouseEnter { .. } => Some(fui_core::InputEvent::CursorEntered {}),

        Event::MouseLeave { .. } => Some(fui_core::InputEvent::CursorLeft {}),

        Event::MouseMove { position, .. } => Some(fui_core::InputEvent::CursorMoved {
            position: fui_core::Point::new(position.x, position.y),
        }),

        Event::MouseButton { state, button } => Some(fui_core::InputEvent::MouseInput {
            state: match state {
                fui_system_core::ElementState::Pressed => fui_core::ElementState::Pressed,
                fui_system_core::ElementState::Released => fui_core::ElementState::Released,
            },

            button: match button {
                fui_system_core::MouseButton::Left => fui_core::MouseButton::Left,
                fui_system_core::MouseButton::Right => fui_core::MouseButton::Right,
                fui_system_core::MouseButton::Middle => fui_core::MouseButton::Middle,
                fui_system_core::MouseButton::Other(other) => fui_core::MouseButton::Other(*other),
            },
        }),

        Event::ScrollWheel { delta } => Some(fui_core::InputEvent::ScrollWheel {
            delta: match delta {
                ScrollDelta::LineDelta(x, y) => fui_core::ScrollDelta::LineDelta(*x, *y),
                ScrollDelta::PixelDelta(x, y) => fui_core::ScrollDelta::PixelDelta(*x, *y),
            },
        }),

        Event::KeyEvent {
            state,
            keycode,
            is_repeat,
            modifiers,
            text,
        } => Some(fui_core::InputEvent::KeyboardInput(fui_core::KeyEvent {
            state: match state {
                fui_system_core::ElementState::Pressed => fui_core::KeyState::Pressed,
                fui_system_core::ElementState::Released => fui_core::KeyState::Released,
            },

            keycode: match keycode {
                Some(fui_system_core::Keycode::Esc) => Some(fui_core::Keycode::Esc),
                Some(fui_system_core::Keycode::F1) => Some(fui_core::Keycode::F1),
                Some(fui_system_core::Keycode::F2) => Some(fui_core::Keycode::F2),
                Some(fui_system_core::Keycode::F3) => Some(fui_core::Keycode::F3),
                Some(fui_system_core::Keycode::F4) => Some(fui_core::Keycode::F4),
                Some(fui_system_core::Keycode::F5) => Some(fui_core::Keycode::F5),
                Some(fui_system_core::Keycode::F6) => Some(fui_core::Keycode::F6),
                Some(fui_system_core::Keycode::F7) => Some(fui_core::Keycode::F7),
                Some(fui_system_core::Keycode::F8) => Some(fui_core::Keycode::F8),
                Some(fui_system_core::Keycode::F9) => Some(fui_core::Keycode::F9),
                Some(fui_system_core::Keycode::F10) => Some(fui_core::Keycode::F10),
                Some(fui_system_core::Keycode::F11) => Some(fui_core::Keycode::F11),
                Some(fui_system_core::Keycode::F12) => Some(fui_core::Keycode::F12),
                Some(fui_system_core::Keycode::PrintScreen) => Some(fui_core::Keycode::PrintScreen),
                Some(fui_system_core::Keycode::ScrollLock) => Some(fui_core::Keycode::ScrollLock),
                Some(fui_system_core::Keycode::Pause) => Some(fui_core::Keycode::Pause),
                Some(fui_system_core::Keycode::Insert) => Some(fui_core::Keycode::Insert),
                Some(fui_system_core::Keycode::Delete) => Some(fui_core::Keycode::Delete),
                Some(fui_system_core::Keycode::Home) => Some(fui_core::Keycode::Home),
                Some(fui_system_core::Keycode::End) => Some(fui_core::Keycode::End),
                Some(fui_system_core::Keycode::PageUp) => Some(fui_core::Keycode::PageUp),
                Some(fui_system_core::Keycode::PageDown) => Some(fui_core::Keycode::PageDown),
                Some(fui_system_core::Keycode::Left) => Some(fui_core::Keycode::Left),
                Some(fui_system_core::Keycode::Right) => Some(fui_core::Keycode::Right),
                Some(fui_system_core::Keycode::Up) => Some(fui_core::Keycode::Up),
                Some(fui_system_core::Keycode::Down) => Some(fui_core::Keycode::Down),
                Some(fui_system_core::Keycode::Backspace) => Some(fui_core::Keycode::Backspace),
                Some(fui_system_core::Keycode::Tab) => Some(fui_core::Keycode::Tab),
                Some(fui_system_core::Keycode::CapsLock) => Some(fui_core::Keycode::CapsLock),
                Some(fui_system_core::Keycode::Enter) => Some(fui_core::Keycode::Enter),
                Some(fui_system_core::Keycode::Shift) => Some(fui_core::Keycode::Shift),
                Some(fui_system_core::Keycode::Ctrl) => Some(fui_core::Keycode::Ctrl),
                Some(fui_system_core::Keycode::Alt) => Some(fui_core::Keycode::Alt),
                Some(fui_system_core::Keycode::Win) => Some(fui_core::Keycode::Win),
                Some(fui_system_core::Keycode::NumLock) => Some(fui_core::Keycode::NumLock),
                _ => None,
            },

            is_repeat: *is_repeat,

            text: text.clone(),

            modifiers: fui_core::KeyModifiers {
                shift: modifiers.shift,
                ctrl: modifiers.ctrl,
                alt: modifiers.alt,
                win: modifiers.win,
                keypad: modifiers.keypad,
                right: modifiers.right,
            },
        })),

        _ => None,
    }
}
