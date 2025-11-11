use windowing_api::{Event, ScrollDelta};

pub fn convert_event(event: &Event) -> Option<fui_core::InputEvent> {
    match event {
        Event::MouseEnter { .. } => Some(fui_core::InputEvent::CursorEntered {}),

        Event::MouseLeave { .. } => Some(fui_core::InputEvent::CursorLeft {}),

        Event::MouseMove { position, .. } => Some(fui_core::InputEvent::CursorMoved {
            position: fui_core::Point::new(position.x, position.y),
        }),

        Event::MouseButton { state, button } => Some(fui_core::InputEvent::MouseInput {
            state: match state {
                windowing_api::ElementState::Pressed => fui_core::ElementState::Pressed,
                windowing_api::ElementState::Released => fui_core::ElementState::Released,
            },

            button: match button {
                windowing_api::MouseButton::Left => fui_core::MouseButton::Left,
                windowing_api::MouseButton::Right => fui_core::MouseButton::Right,
                windowing_api::MouseButton::Middle => fui_core::MouseButton::Middle,
                windowing_api::MouseButton::Other(other) => fui_core::MouseButton::Other(*other),
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
                windowing_api::ElementState::Pressed => fui_core::KeyState::Pressed,
                windowing_api::ElementState::Released => fui_core::KeyState::Released,
            },

            keycode: match keycode {
                Some(windowing_api::Keycode::Esc) => Some(fui_core::Keycode::Esc),
                Some(windowing_api::Keycode::F1) => Some(fui_core::Keycode::F1),
                Some(windowing_api::Keycode::F2) => Some(fui_core::Keycode::F2),
                Some(windowing_api::Keycode::F3) => Some(fui_core::Keycode::F3),
                Some(windowing_api::Keycode::F4) => Some(fui_core::Keycode::F4),
                Some(windowing_api::Keycode::F5) => Some(fui_core::Keycode::F5),
                Some(windowing_api::Keycode::F6) => Some(fui_core::Keycode::F6),
                Some(windowing_api::Keycode::F7) => Some(fui_core::Keycode::F7),
                Some(windowing_api::Keycode::F8) => Some(fui_core::Keycode::F8),
                Some(windowing_api::Keycode::F9) => Some(fui_core::Keycode::F9),
                Some(windowing_api::Keycode::F10) => Some(fui_core::Keycode::F10),
                Some(windowing_api::Keycode::F11) => Some(fui_core::Keycode::F11),
                Some(windowing_api::Keycode::F12) => Some(fui_core::Keycode::F12),
                Some(windowing_api::Keycode::PrintScreen) => Some(fui_core::Keycode::PrintScreen),
                Some(windowing_api::Keycode::ScrollLock) => Some(fui_core::Keycode::ScrollLock),
                Some(windowing_api::Keycode::Pause) => Some(fui_core::Keycode::Pause),
                Some(windowing_api::Keycode::Insert) => Some(fui_core::Keycode::Insert),
                Some(windowing_api::Keycode::Delete) => Some(fui_core::Keycode::Delete),
                Some(windowing_api::Keycode::Home) => Some(fui_core::Keycode::Home),
                Some(windowing_api::Keycode::End) => Some(fui_core::Keycode::End),
                Some(windowing_api::Keycode::PageUp) => Some(fui_core::Keycode::PageUp),
                Some(windowing_api::Keycode::PageDown) => Some(fui_core::Keycode::PageDown),
                Some(windowing_api::Keycode::Left) => Some(fui_core::Keycode::Left),
                Some(windowing_api::Keycode::Right) => Some(fui_core::Keycode::Right),
                Some(windowing_api::Keycode::Up) => Some(fui_core::Keycode::Up),
                Some(windowing_api::Keycode::Down) => Some(fui_core::Keycode::Down),
                Some(windowing_api::Keycode::Backspace) => Some(fui_core::Keycode::Backspace),
                Some(windowing_api::Keycode::Tab) => Some(fui_core::Keycode::Tab),
                Some(windowing_api::Keycode::CapsLock) => Some(fui_core::Keycode::CapsLock),
                Some(windowing_api::Keycode::Enter) => Some(fui_core::Keycode::Enter),
                Some(windowing_api::Keycode::Shift) => Some(fui_core::Keycode::Shift),
                Some(windowing_api::Keycode::Ctrl) => Some(fui_core::Keycode::Ctrl),
                Some(windowing_api::Keycode::Alt) => Some(fui_core::Keycode::Alt),
                Some(windowing_api::Keycode::Win) => Some(fui_core::Keycode::Win),
                Some(windowing_api::Keycode::NumLock) => Some(fui_core::Keycode::NumLock),
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
