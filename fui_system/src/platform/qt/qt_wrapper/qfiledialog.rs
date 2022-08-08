use crate::platform::qt::qt_wrapper::{QString, QStringList};

pub struct QFileDialog;

impl QFileDialog {
    pub fn get_open_file_name(
        caption: QString,
        dir: QString,
        filter: QString,
        selected_filter: &mut QString,
        options: i32,
    ) -> Option<QString> {
        unsafe {
            let qstring_ptr = crate::platform::qt::qt_wrapper::QFileDialog_getOpenFileName(
                std::ptr::null_mut(),
                caption.this,
                dir.this,
                filter.this,
                selected_filter.this,
                options,
            );
            if qstring_ptr.is_null() {
                None
            } else {
                Some(QString {
                    this: qstring_ptr,
                    is_owned: true,
                })
            }
        }
    }

    pub fn get_open_file_names(
        caption: QString,
        dir: QString,
        filter: QString,
        selected_filter: &mut QString,
        options: i32,
    ) -> QStringList {
        unsafe {
            let qstringlist_ptr = crate::platform::qt::qt_wrapper::QFileDialog_getOpenFileNames(
                std::ptr::null_mut(),
                caption.this,
                dir.this,
                filter.this,
                selected_filter.this,
                options,
            );
            QStringList {
                this: qstringlist_ptr,
            }
        }
    }

    pub fn get_existing_directory(caption: QString, dir: QString, options: i32) -> Option<QString> {
        unsafe {
            let qstring_ptr = crate::platform::qt::qt_wrapper::QFileDialog_getExistingDirectory(
                std::ptr::null_mut(),
                caption.this,
                dir.this,
                options,
            );
            if qstring_ptr.is_null() {
                None
            } else {
                Some(QString {
                    this: qstring_ptr,
                    is_owned: true,
                })
            }
        }
    }

    pub fn get_save_file_name(
        caption: QString,
        dir: QString,
        filter: QString,
        selected_filter: &mut QString,
        options: i32,
    ) -> Option<QString> {
        unsafe {
            let qstring_ptr = crate::platform::qt::qt_wrapper::QFileDialog_getSaveFileName(
                std::ptr::null_mut(),
                caption.this,
                dir.this,
                filter.this,
                selected_filter.this,
                options,
            );
            if qstring_ptr.is_null() {
                None
            } else {
                Some(QString {
                    this: qstring_ptr,
                    is_owned: true,
                })
            }
        }
    }
}
