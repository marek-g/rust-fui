/// Event enum.
#[repr(C)]
#[derive(Debug)]
pub enum Event {
    MouseEnter,
    MouseLeave,
    MouseButtonPress { button: MouseButton },
    MouseButtonRelease { button: MouseButton },
    MouseMove { position: Position },
}

#[repr(C)]
#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[repr(C)]
#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Event {
    /// Allocates new Event enum. Can be called from C.
    #[no_mangle]
    pub extern "C" fn alloc_event() -> *mut Event {
        Box::into_raw(Box::new(Event::MouseEnter))
    }
}
