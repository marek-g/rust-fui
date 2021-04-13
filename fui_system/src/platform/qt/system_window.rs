use crate::platform::qt::qt_wrapper::{QString, QWindow};

pub struct SystemWindow {
    qwindow: QWindow,
}

impl SystemWindow {
    pub fn new(parent: Option<&mut SystemWindow>) -> Result<Self, ()> {
        let qwindow = QWindow::new(parent.map(|p| &mut p.qwindow))?;
        Ok(Self { qwindow })
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), ()> {
        let title = QString::from_str(title)?;
        self.qwindow.set_title(&title);
        Ok(())
    }

    pub fn set_visible(&mut self, visible: bool) -> Result<(), ()> {
        self.qwindow.set_visible(visible);
        Ok(())
    }
}
