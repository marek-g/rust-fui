use crate::platform::qt::qt_wrapper::QFileDialog;
use crate::Window;
use std::path::PathBuf;

pub struct FileDialog;

impl FileDialog {
    pub fn get_open_file_name(parent: Option<&mut Window>, caption: Option<&str>) -> PathBuf {
        QFileDialog::get_open_file_name();
        PathBuf::new()
    }
}
