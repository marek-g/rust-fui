use crate::platform::qt::qt_wrapper::{QApplication, QFileDialog};
use crate::Application;
use crate::Window;
use std::path::PathBuf;
use std::sync::mpsc::channel;

pub struct FileDialog;

impl FileDialog {
    /// Can be called only from GUI thread.
    pub fn get_open_file_name(parent: Option<&mut Window>, caption: Option<&str>) -> PathBuf {
        if !Application::is_gui_thread() {
            panic!("FileDialog::get_open_file_name can be called only from GUI thread");
        }

        QFileDialog::get_open_file_name();
        PathBuf::new()
    }
}
