use crate::common::Point;

#[derive(Clone, Debug, PartialEq)]
pub enum ControlEvent {
    HoverEnter,
    HoverLeave,

    FocusEnter,
    FocusLeave,

    TapDown { position: Point },
    TapUp { position: Point },
    TapMove { position: Point },
}
