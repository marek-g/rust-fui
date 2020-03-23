use crate::common::Point;
use crate::events::key_event::KeyEvent;

#[derive(Clone, Debug, PartialEq)]
pub enum ControlEvent {
    HoverEnter,
    HoverLeave,

    FocusEnter,
    FocusLeave,

    TapDown { position: Point },
    TapUp { position: Point },
    TapMove { position: Point },

    KeyboardInput(KeyEvent),
}
