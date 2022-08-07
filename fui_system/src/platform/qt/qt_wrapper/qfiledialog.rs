use crate::platform::qt::qt_wrapper::QString;

pub struct QFileDialog;

impl QFileDialog {
    pub fn get_open_file_name() -> Option<QString> {
        let caption = QString::from_str("Caption").unwrap();
        let dir = QString::from_str("").unwrap();
        let filter = QString::from_str("All Files (*.*)").unwrap();
        let selected_filter = QString::from_str("").unwrap();
        unsafe {
            let qstring_ptr = crate::platform::qt::qt_wrapper::QFileDialog_getOpenFileName(
                std::ptr::null_mut(),
                caption.this,
                dir.this,
                filter.this,
                selected_filter.this,
                0x0,
            );
            if qstring_ptr.is_null() {
                None
            } else {
                Some(QString { this: qstring_ptr })
            }
        }
    }
}
