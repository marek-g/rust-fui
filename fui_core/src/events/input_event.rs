use crate::common::Point;
use crate::events::key_event::KeyEvent;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
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

    /// Mouse scroll wheel rolled or touchpad scroll gesture.
    ScrollWheel {
        delta: ScrollDelta,
    },

    KeyboardInput(KeyEvent),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ElementState {
    Pressed,
    Released,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// Scroll delta enum.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ScrollDelta {
    /// Amount of lines to scroll horizontally and vertically.
    /// This is generated by mouse wheel.
    LineDelta(f32, f32),

    /// Amount of pixels to scroll horizontally and vertically.
    /// This is generated by touchpad.
    PixelDelta(f32, f32),
}
