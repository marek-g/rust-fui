use crate::common::Event;
use crate::platform::qt::qt_wrapper::{QString, QWindow};
use crate::{FUISystemError, Icon};
use std::ffi::c_void;

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
    /// Sets the visibility of the window.
    ///
    pub fn set_visible(&mut self, visible: bool) -> Result<(), FUISystemError> {
        self.qwindow.set_visible(visible);
        Ok(())
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
    /// Marks the entire window as dirty and schedules a repaint.
    /// Subsequent calls to this function before the next paint event will get ignored.
    ///
    pub fn update(&mut self) {
        self.qwindow.update();
    }

    pub fn on_event<F: 'static + FnMut(&Event) -> bool>(&mut self, callback: F) {
        self.qwindow.on_event(callback);
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
        Ok(context.get_proc_address(proc_name)?)
    }
}
