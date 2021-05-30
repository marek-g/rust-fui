use crate::common::Point;
use crate::events::key_event::KeyEvent;

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum ElementState {
    Pressed,
    Released,
}

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}
