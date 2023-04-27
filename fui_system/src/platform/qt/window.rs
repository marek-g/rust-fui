use crate::platform::qt::qt_wrapper::{
    FFIElementState, FFIEvent, FFIKeyModifiers, FFIMouseButton, FFIPosition, FFIScrollDelta,
    QString, QWindow,
};
use crate::{FUISystemError, Icon};
use fui_system_core::{
    CursorShape, Edge, ElementState, Event, KeyModifiers, Keycode, MouseButton, Position,
    ScrollDelta, TranslucentEffect, WindowFrameType,
};
use std::ffi::{c_void, CStr};

///
/// Represents a window in the underlying windowing system.
///
pub struct Window {
    qwindow: QWindow,
}

impl Window {
    ///
    /// Creates a window as a child of the given parent window.
    ///
    pub fn new(parent: Option<&mut Window>) -> Result<Self, FUISystemError> {
        let qwindow = QWindow::new(parent.map(|p| &mut p.qwindow))?;
        Ok(Self { qwindow })
    }

    ///
    /// Sets the window's title.
    ///
    pub fn set_title(&mut self, title: &str) -> Result<(), FUISystemError> {
        let title = QString::from_str(title)?;
        self.qwindow.set_title(&title);
        Ok(())
    }

    ///
    /// Sets the window's icon.
    ///
    pub fn set_icon(&mut self, icon: &Icon) -> Result<(), FUISystemError> {
        self.qwindow.set_icon(&icon.qicon);
        Ok(())
    }

    ///
    /// Sets if window should stay on top.
    ///
    pub fn set_stay_on_top(&mut self, stay_on_top: bool) -> Result<(), FUISystemError> {
        self.qwindow.set_stay_on_top(stay_on_top);
        Ok(())
    }

    ///
    /// Sets if window should be transparent for input.
    ///
    pub fn set_transparent_for_input(
        &mut self,
        transparent_for_input: bool,
    ) -> Result<(), FUISystemError> {
        self.qwindow
            .set_transparent_for_input(transparent_for_input);
        Ok(())
    }

    ///
    /// Sets window frame type.
    ///
    pub fn set_frame_type(&mut self, frame_type: WindowFrameType) -> Result<(), FUISystemError> {
        let frame_type = match frame_type {
            WindowFrameType::Frameless => 0,
            WindowFrameType::Normal => 1,
        };
        self.qwindow.set_frame_type(frame_type);
        Ok(())
    }

    ///
    /// Makes the window popup window.
    ///
    pub fn set_popup_window(&mut self) {
        self.qwindow.set_popup_window();
    }

    ///
    /// Sets if window should have an alpha channel for translucent regions.
    ///
    pub fn set_translucent_background(
        &mut self,
        translucent_effect: TranslucentEffect,
    ) -> Result<(), FUISystemError> {
        let translucent_effect = match translucent_effect {
            TranslucentEffect::None => 0,
            TranslucentEffect::Transparent => 1,
            TranslucentEffect::Blur => 2,
        };
        self.qwindow.set_translucent_background(translucent_effect);
        Ok(())
    }

    ///
    /// Sets the visibility of the window.
    ///
    pub fn set_visible(&mut self, visible: bool) -> Result<(), FUISystemError> {
        self.qwindow.set_visible(visible);
        Ok(())
    }

    ///
    /// Gets window position (excluding it's window frame).
    ///
    pub fn get_position(&mut self) -> (i32, i32) {
        self.qwindow.get_position()
    }

    ///
    /// Sets window position (excluding it's window frame).
    ///
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.qwindow.set_position(x, y)
    }

    ///
    /// Gets window position (including it's window frame).
    ///
    pub fn get_frame_position(&mut self) -> (i32, i32) {
        self.qwindow.get_frame_position()
    }

    ///
    /// Sets window position (including it's window frame).
    ///
    pub fn set_frame_position(&mut self, x: i32, y: i32) {
        self.qwindow.set_frame_position(x, y)
    }

    ///
    /// Get window width, excluding any window frame.
    ///
    pub fn get_width(&mut self) -> i32 {
        self.qwindow.get_width()
    }

    ///
    /// Get window height, excluding any window frame.
    ///
    pub fn get_height(&mut self) -> i32 {
        self.qwindow.get_height()
    }

    ///
    /// Resize window, excluding any window frame.
    ///
    pub fn resize(&mut self, width: i32, height: i32) {
        self.qwindow.resize(width, height);
    }

    ///
    /// Sets minimum window size, excluding any window frame.
    ///
    pub fn set_minimum_size(&mut self, width: i32, height: i32) {
        self.qwindow.set_minimum_size(width, height);
    }

    ///
    /// Sets the cursor shape for this window.
    ///
    pub fn set_cursor(&mut self, cursor_shape: CursorShape) {
        let cursor_shape = match cursor_shape {
            CursorShape::ArrowCursor => 0,
            CursorShape::UpArrowCursor => 1,
            CursorShape::CrossCursor => 2,
            CursorShape::WaitCursor => 3,
            CursorShape::IBeamCursor => 4,
            CursorShape::SizeVerCursor => 5,
            CursorShape::SizeHorCursor => 6,
            CursorShape::SizeBDiagCursor => 7,
            CursorShape::SizeFDiagCursor => 8,
            CursorShape::SizeAllCursor => 9,
            CursorShape::BlankCursor => 10,
            CursorShape::SplitVCursor => 11,
            CursorShape::SplitHCursor => 12,
            CursorShape::PointingHandCursor => 13,
            CursorShape::ForbiddenCursor => 14,
            CursorShape::OpenHandCursor => 17,
            CursorShape::ClosedHandCursor => 18,
            CursorShape::WhatsThisCursor => 15,
            CursorShape::BusyCursor => 16,
            CursorShape::DragMoveCursor => 20,
            CursorShape::DragCopyCursor => 19,
            CursorShape::DragLinkCursor => 21,
        };
        self.qwindow.set_cursor_shape(cursor_shape);
    }

    ///
    /// Start a system-specific move operation.
    /// Returns true if the operation was supported by the system.
    ///
    pub fn start_system_move(&mut self) -> bool {
        self.qwindow.start_system_move()
    }

    ///
    /// Start a system-specific resize operation.
    /// Returns true if the operation was supported by the system.
    ///
    pub fn start_system_resize(&mut self, edges: Edge) -> bool {
        self.qwindow.start_system_resize(edges.bits())
    }

    ///
    /// Marks the entire window as dirty and schedules a repaint.
    /// Subsequent calls to this function before the next paint event will get ignored.
    ///
    pub fn update(&mut self) {
        self.qwindow.update();
    }

    pub fn on_event<F: 'static + FnMut(Event) -> bool>(&mut self, mut callback: F) {
        self.qwindow.on_event(move |ffi_event: &FFIEvent| {
            if let Some(event) = convert_event(ffi_event) {
                callback(event)
            } else {
                false
            }
        });
    }

    ///
    /// OpenGL.
    ///
    /// Sets the callback that is called whenever the window contents needs to be repainted.
    /// The OpenGL context of the window is already made current.
    ///
    pub fn on_paint_gl<F: 'static + FnMut()>(&mut self, callback: F) {
        self.qwindow.on_paint_gl(callback);
    }

    pub fn get_opengl_proc_address(
        &self,
        proc_name: &str,
    ) -> Result<*const c_void, FUISystemError> {
        let context = self.qwindow.get_context()?;
        context.get_proc_address(proc_name)
    }
}

fn convert_event(ffi_event: &FFIEvent) -> Option<Event> {
    match ffi_event {
        FFIEvent::MouseEnter => Some(Event::MouseEnter),
        FFIEvent::MouseLeave => Some(Event::MouseLeave),
        FFIEvent::MouseButton { state, button } => Some(Event::MouseButton {
            state: convert_element_state(state),
            button: convert_mouse_button(button),
        }),
        FFIEvent::MouseMove { position } => Some(Event::MouseMove {
            position: convert_position(position),
        }),
        FFIEvent::ScrollWheel { delta } => Some(Event::ScrollWheel {
            delta: convert_delta(delta),
        }),
        FFIEvent::KeyEvent {
            state,
            is_repeat,
            keycode,
            modifiers,
            text,
        } => Some(Event::KeyEvent {
            state: convert_element_state(state),
            is_repeat: *is_repeat,
            keycode: convert_keycode(*keycode, modifiers),
            modifiers: convert_modifiers(*keycode, modifiers),
            text: convert_text(*text),
        }),
        FFIEvent::Resize { width, height } => Some(Event::Resize {
            width: *width,
            height: *height,
        }),
    }
}

fn convert_element_state(state: &FFIElementState) -> ElementState {
    match state {
        FFIElementState::Pressed => ElementState::Pressed,
        FFIElementState::Released => ElementState::Released,
    }
}

fn convert_mouse_button(button: &FFIMouseButton) -> MouseButton {
    match button {
        FFIMouseButton::Left => MouseButton::Left,
        FFIMouseButton::Right => MouseButton::Right,
        FFIMouseButton::Middle => MouseButton::Middle,
        FFIMouseButton::Other(code) => MouseButton::Other(*code),
    }
}

fn convert_position(position: &FFIPosition) -> Position {
    Position {
        x: position.x,
        y: position.y,
    }
}

fn convert_delta(delta: &FFIScrollDelta) -> ScrollDelta {
    match delta {
        FFIScrollDelta::LineDelta(x, y) => ScrollDelta::LineDelta(*x, *y),
        FFIScrollDelta::PixelDelta(x, y) => ScrollDelta::PixelDelta(*x, *y),
    }
}

fn convert_keycode(keycode: i32, _modifiers: &FFIKeyModifiers) -> Option<Keycode> {
    match keycode {
        0x01000000 => Some(Keycode::Esc),
        0x01000030 => Some(Keycode::F1),
        0x01000031 => Some(Keycode::F2),
        0x01000032 => Some(Keycode::F3),
        0x01000033 => Some(Keycode::F4),
        0x01000034 => Some(Keycode::F5),
        0x01000035 => Some(Keycode::F6),
        0x01000036 => Some(Keycode::F7),
        0x01000037 => Some(Keycode::F8),
        0x01000038 => Some(Keycode::F9),
        0x01000039 => Some(Keycode::F10),
        0x0100003a => Some(Keycode::F11),
        0x0100003b => Some(Keycode::F12),
        0x01000009 => Some(Keycode::PrintScreen),
        0x01000026 => Some(Keycode::ScrollLock),
        0x01000008 => Some(Keycode::Pause),
        0x01000006 => Some(Keycode::Insert),
        0x01000007 => Some(Keycode::Delete),
        0x01000010 => Some(Keycode::Home),
        0x01000011 => Some(Keycode::End),
        0x01000016 => Some(Keycode::PageUp),
        0x01000017 => Some(Keycode::PageDown),
        0x01000012 => Some(Keycode::Left),
        0x01000014 => Some(Keycode::Right),
        0x01000013 => Some(Keycode::Up),
        0x01000015 => Some(Keycode::Down),
        0x01000003 => Some(Keycode::Backspace),
        0x01000001 => Some(Keycode::Tab),
        0x01000024 => Some(Keycode::CapsLock),
        0x01000004 => Some(Keycode::Enter),
        0x01000005 => Some(Keycode::Enter),
        0x01000020 => Some(Keycode::Shift),
        0x01000021 => Some(Keycode::Ctrl),
        0x01000023 => Some(Keycode::Alt),
        0x01001103 => Some(Keycode::Alt),
        0x01000022 => Some(Keycode::Win),
        0x01000025 => Some(Keycode::NumLock),
        _ => None,
    }
}

fn convert_modifiers(keycode: i32, modifiers: &FFIKeyModifiers) -> KeyModifiers {
    KeyModifiers {
        alt: modifiers.alt,
        ctrl: modifiers.ctrl,
        shift: modifiers.shift,
        win: modifiers.win,
        keypad: modifiers.keypad,
        right: keycode == 0x01001103,
    }
}

fn convert_text(text: *const i8) -> Option<String> {
    if text.is_null() {
        None
    } else {
        let text = unsafe { CStr::from_ptr(text) };
        Some(text.to_str().unwrap().to_owned())
    }
}
