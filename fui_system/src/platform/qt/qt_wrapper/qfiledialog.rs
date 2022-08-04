use crate::platform::qt::qt_wrapper::QString;

pub struct QFileDialog;

impl QFileDialog {
    pub fn get_open_file_name() -> QString {
        let caption = QString::from_str("Caption").unwrap();
        let dir = QString::from_str("").unwrap();
        let filter = QString::from_str("").unwrap();
        let selected_filter = QString::from_str("").unwrap();
        unsafe {
            QString {
                this: crate::platform::qt::qt_wrapper::QFileDialog_getOpenFileName(
                    std::ptr::null_mut(),
                    caption.this,
                    dir.this,
                    filter.this,
                    selected_filter.this,
                    0,
                ),
            }
        }
    }
}
