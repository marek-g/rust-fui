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
                // function row
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
                Some(windowing_api::Keycode::F13) => Some(fui_core::Keycode::F13),
                Some(windowing_api::Keycode::F14) => Some(fui_core::Keycode::F14),
                Some(windowing_api::Keycode::F15) => Some(fui_core::Keycode::F15),
                Some(windowing_api::Keycode::F16) => Some(fui_core::Keycode::F16),
                Some(windowing_api::Keycode::F17) => Some(fui_core::Keycode::F17),
                Some(windowing_api::Keycode::F18) => Some(fui_core::Keycode::F18),
                Some(windowing_api::Keycode::F19) => Some(fui_core::Keycode::F19),
                Some(windowing_api::Keycode::F20) => Some(fui_core::Keycode::F20),
                Some(windowing_api::Keycode::PrintScreen) => Some(fui_core::Keycode::PrintScreen),
                Some(windowing_api::Keycode::ScrollLock) => Some(fui_core::Keycode::ScrollLock),
                Some(windowing_api::Keycode::Pause) => Some(fui_core::Keycode::Pause),

                // navigation
                Some(windowing_api::Keycode::Insert) => Some(fui_core::Keycode::Insert),
                Some(windowing_api::Keycode::Delete) => Some(fui_core::Keycode::Delete),
                Some(windowing_api::Keycode::Home) => Some(fui_core::Keycode::Home),
                Some(windowing_api::Keycode::End) => Some(fui_core::Keycode::End),
                Some(windowing_api::Keycode::PageUp) => Some(fui_core::Keycode::PageUp),
                Some(windowing_api::Keycode::PageDown) => Some(fui_core::Keycode::PageDown),

                // arrows
                Some(windowing_api::Keycode::Left) => Some(fui_core::Keycode::Left),
                Some(windowing_api::Keycode::Right) => Some(fui_core::Keycode::Right),
                Some(windowing_api::Keycode::Up) => Some(fui_core::Keycode::Up),
                Some(windowing_api::Keycode::Down) => Some(fui_core::Keycode::Down),

                // modifiers
                Some(windowing_api::Keycode::Shift) => Some(fui_core::Keycode::Shift),
                Some(windowing_api::Keycode::Ctrl) => Some(fui_core::Keycode::Ctrl),
                Some(windowing_api::Keycode::Win) => Some(fui_core::Keycode::Win),
                Some(windowing_api::Keycode::Alt) => Some(fui_core::Keycode::Alt),
                Some(windowing_api::Keycode::AltGr) => Some(fui_core::Keycode::AltGr),

                // special
                Some(windowing_api::Keycode::Menu) => Some(fui_core::Keycode::Menu),
                Some(windowing_api::Keycode::Backspace) => Some(fui_core::Keycode::Backspace),
                Some(windowing_api::Keycode::Tab) => Some(fui_core::Keycode::Tab),
                Some(windowing_api::Keycode::CapsLock) => Some(fui_core::Keycode::CapsLock),
                Some(windowing_api::Keycode::Return) => Some(fui_core::Keycode::Return),
                Some(windowing_api::Keycode::Enter) => Some(fui_core::Keycode::Enter),
                Some(windowing_api::Keycode::Space) => Some(fui_core::Keycode::Space),

                // symbols
                Some(windowing_api::Keycode::QuoteLeft) => Some(fui_core::Keycode::QuoteLeft),
                Some(windowing_api::Keycode::Minus) => Some(fui_core::Keycode::Minus),
                Some(windowing_api::Keycode::Equal) => Some(fui_core::Keycode::Equal),
                Some(windowing_api::Keycode::Asterisk) => Some(fui_core::Keycode::Asterisk),
                Some(windowing_api::Keycode::Plus) => Some(fui_core::Keycode::Plus),
                Some(windowing_api::Keycode::Period) => Some(fui_core::Keycode::Period),
                Some(windowing_api::Keycode::Slash) => Some(fui_core::Keycode::Slash),
                Some(windowing_api::Keycode::BracketLeft) => Some(fui_core::Keycode::BracketLeft),
                Some(windowing_api::Keycode::BracketRight) => Some(fui_core::Keycode::BracketRight),
                Some(windowing_api::Keycode::Backslash) => Some(fui_core::Keycode::Backslash),
                Some(windowing_api::Keycode::Semicolon) => Some(fui_core::Keycode::Semicolon),
                Some(windowing_api::Keycode::Quote) => Some(fui_core::Keycode::Quote),
                Some(windowing_api::Keycode::Comma) => Some(fui_core::Keycode::Comma),

                // letters
                Some(windowing_api::Keycode::KeyA) => Some(fui_core::Keycode::KeyA),
                Some(windowing_api::Keycode::KeyB) => Some(fui_core::Keycode::KeyB),
                Some(windowing_api::Keycode::KeyC) => Some(fui_core::Keycode::KeyC),
                Some(windowing_api::Keycode::KeyD) => Some(fui_core::Keycode::KeyD),
                Some(windowing_api::Keycode::KeyE) => Some(fui_core::Keycode::KeyE),
                Some(windowing_api::Keycode::KeyF) => Some(fui_core::Keycode::KeyF),
                Some(windowing_api::Keycode::KeyG) => Some(fui_core::Keycode::KeyG),
                Some(windowing_api::Keycode::KeyH) => Some(fui_core::Keycode::KeyH),
                Some(windowing_api::Keycode::KeyI) => Some(fui_core::Keycode::KeyI),
                Some(windowing_api::Keycode::KeyJ) => Some(fui_core::Keycode::KeyJ),
                Some(windowing_api::Keycode::KeyK) => Some(fui_core::Keycode::KeyK),
                Some(windowing_api::Keycode::KeyL) => Some(fui_core::Keycode::KeyL),
                Some(windowing_api::Keycode::KeyM) => Some(fui_core::Keycode::KeyM),
                Some(windowing_api::Keycode::KeyN) => Some(fui_core::Keycode::KeyN),
                Some(windowing_api::Keycode::KeyO) => Some(fui_core::Keycode::KeyO),
                Some(windowing_api::Keycode::KeyP) => Some(fui_core::Keycode::KeyP),
                Some(windowing_api::Keycode::KeyQ) => Some(fui_core::Keycode::KeyQ),
                Some(windowing_api::Keycode::KeyR) => Some(fui_core::Keycode::KeyR),
                Some(windowing_api::Keycode::KeyS) => Some(fui_core::Keycode::KeyS),
                Some(windowing_api::Keycode::KeyT) => Some(fui_core::Keycode::KeyT),
                Some(windowing_api::Keycode::KeyU) => Some(fui_core::Keycode::KeyU),
                Some(windowing_api::Keycode::KeyV) => Some(fui_core::Keycode::KeyV),
                Some(windowing_api::Keycode::KeyW) => Some(fui_core::Keycode::KeyW),
                Some(windowing_api::Keycode::KeyX) => Some(fui_core::Keycode::KeyX),
                Some(windowing_api::Keycode::KeyY) => Some(fui_core::Keycode::KeyY),
                Some(windowing_api::Keycode::KeyZ) => Some(fui_core::Keycode::KeyZ),

                // digits
                Some(windowing_api::Keycode::Key0) => Some(fui_core::Keycode::Key0),
                Some(windowing_api::Keycode::Key1) => Some(fui_core::Keycode::Key1),
                Some(windowing_api::Keycode::Key2) => Some(fui_core::Keycode::Key2),
                Some(windowing_api::Keycode::Key3) => Some(fui_core::Keycode::Key3),
                Some(windowing_api::Keycode::Key4) => Some(fui_core::Keycode::Key4),
                Some(windowing_api::Keycode::Key5) => Some(fui_core::Keycode::Key5),
                Some(windowing_api::Keycode::Key6) => Some(fui_core::Keycode::Key6),
                Some(windowing_api::Keycode::Key7) => Some(fui_core::Keycode::Key7),
                Some(windowing_api::Keycode::Key8) => Some(fui_core::Keycode::Key8),
                Some(windowing_api::Keycode::Key9) => Some(fui_core::Keycode::Key9),

                // numeric keyboard
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
