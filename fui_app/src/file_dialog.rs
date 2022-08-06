use crate::Application;
use crate::Window;
use std::path::PathBuf;

pub struct FileDialog;

impl FileDialog {
    pub fn get_open_file_name(parent: Option<&mut Window>, caption: Option<&str>) -> PathBuf {
        if fui_system::Application::is_gui_thread() {
            fui_system::FileDialog::get_open_file_name(None, caption);
            PathBuf::new()
        } else {
            let (sender, receiver) = std::sync::mpsc::channel();
            fui_system::Application::post_func({
                let caption = caption.map(|caption| caption.to_string());
                move || {
                    fui_system::FileDialog::get_open_file_name(None, caption.as_deref());
                    sender.send(PathBuf::new()).unwrap();
                }
            });
            receiver.recv().unwrap()
        }
    }
}
