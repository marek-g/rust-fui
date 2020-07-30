use crate::common::Point;
use crate::events::key_event::KeyEvent;

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

pub enum ElementState {
    Pressed,
    Released,
}

pub enum InputEvent {
    CursorEntered {},

    CursorLeft {},

    CursorMoved {
        position: Point,
    },

    MouseInput {
        state: ElementState,
        button: MouseButton,
    },

    KeyboardInput(KeyEvent),
}
