#[derive(Clone, Debug, PartialEq)]
pub struct KeyEvent {
    pub state: KeyState,
    pub keycode: Option<Keycode>,
    pub text: Option<String>,
    pub modifiers: KeyModifiers,
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keycode {
    Esc,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Left,
    Right,
    Up,
    Down,
    Backspace,
    Tab,
    Enter,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LAlt,
    RAlt,
    LWin,
    RWin,
    NumLock,
    NumpadEnter,
    Copy,
    Paste,
    Cut,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub win: bool,
}
