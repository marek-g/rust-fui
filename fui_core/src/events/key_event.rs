#[derive(Clone, Debug, PartialEq)]
pub struct KeyEvent {
    pub state: KeyState,
    pub keycode: Option<Keycode>,
    pub is_repeat: bool,
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
    // function row
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
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    PrintScreen,
    ScrollLock,
    Pause,

    // navigation
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    // arrows
    Left,
    Right,
    Up,
    Down,

    // modifiers
    Shift,
    Ctrl,
    Win,
    Alt,
    AltGr,

    // special
    Menu,
    Backspace,
    Tab,
    CapsLock,
    Return,
    Enter,
    Space,

    // symbols
    QuoteLeft,
    Minus,
    Equal,
    Asterisk,
    Plus,
    Period,
    Slash,
    BracketLeft,
    BracketRight,
    Backslash,
    Semicolon,
    Quote,
    Comma,

    // letters
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,

    // digits
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    // numeric keyboard
    NumLock,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub win: bool,

    /// does the key belong to the keypad
    pub keypad: bool,

    /// is it the right variant of the key (like right shift, alt, etc.)
    /// this is not yet behaving correctly
    pub right: bool,
}
