use crate::common::Point;
use crate::events::key_event::KeyEvent;
use crate::ScrollDelta;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ControlEvent {
    /// The control is hovered when it's under the cursor
    /// including hit point control and all it's parents.
    HoverChange(bool),

    /// It's true when the cursor is directly over the control.
    HitTestChange(bool),

    FocusChange(bool),

    TapDown {
        position: Point,
    },
    TapUp {
        position: Point,
    },
    TapMove {
        position: Point,
    },

    PointerMove {
        position: Point,
    },

    ScrollWheel {
        delta: ScrollDelta,
    },

    KeyboardInput(KeyEvent),
}
