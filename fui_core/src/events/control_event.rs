use crate::common::Point;
use crate::events::key_event::KeyEvent;

#[derive(Clone, Debug, PartialEq)]
pub enum ControlEvent {
    HoverChange(bool),
    FocusChange(bool),

    TapDown { position: Point },
    TapUp { position: Point },
    TapMove { position: Point },

    PointerMove { position: Point },

    KeyboardInput(KeyEvent),
}
