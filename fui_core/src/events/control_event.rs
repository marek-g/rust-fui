use crate::common::Point;
use crate::events::key_event::KeyEvent;
use crate::ScrollDelta;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ControlEvent {
    HoverChange(bool),
    FocusChange(bool),

    TapDown { position: Point },
    TapUp { position: Point },
    TapMove { position: Point },

    PointerMove { position: Point },

    ScrollWheel { delta: ScrollDelta },

    KeyboardInput(KeyEvent),
}
