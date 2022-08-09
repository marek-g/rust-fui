use crate::platform::qt::qt_wrapper::{QFileDialog, QString};
use crate::Application;
use std::path::{Path, PathBuf};

pub struct FileDialog;

impl FileDialog {
    /// Can be called only from GUI thread.
    pub fn get_open_file_name(
        title: Option<&str>,
        initial_path: Option<&Path>,
        filters: Option<&str>,
    ) -> Option<PathBuf> {
        if !Application::is_gui_thread() {
            panic!("FileDialog::get_open_file_name can be called only from GUI thread");
        }

        let caption = title
            .map_or_else(|| QString::null(), |v| QString::from_str(v))
            .unwrap();

        let dir = initial_path
            .map_or_else(
                || QString::null(),
                |v| QString::from_str(v.to_str().unwrap()),
            )
            .unwrap();

        let filters = filters
            .map_or_else(|| QString::null(), |v| QString::from_str(v))
            .unwrap();

        let mut selected_filter = QString::null().unwrap();

        QFileDialog::get_open_file_name(caption, dir, filters, &mut selected_filter, 0)
            .map(|path| path.as_path_buf().unwrap())
    }

    /// Can be called only from GUI thread.
    pub fn get_open_file_names(
        title: Option<&str>,
        initial_path: Option<&Path>,
        filters: Option<&str>,
    ) -> Vec<PathBuf> {
        if !Application::is_gui_thread() {
            panic!("FileDialog::get_open_file_names can be called only from GUI thread");
        }

        let caption = title
            .map_or_else(|| QString::null(), |v| QString::from_str(v))
            .unwrap();

        let dir = initial_path
            .map_or_else(
                || QString::null(),
                |v| QString::from_str(v.to_str().unwrap()),
            )
            .unwrap();

        let filters = filters
            .map_or_else(|| QString::null(), |v| QString::from_str(v))
            .unwrap();

        let mut selected_filter = QString::null().unwrap();

        let qstringlist =
            QFileDialog::get_open_file_names(caption, dir, filters, &mut selected_filter, 0);
        let mut res = Vec::new();
        for i in 0..qstringlist.size() {
            res.push(qstringlist.get(i).unwrap().into());
        }
        res
    }

    /// Can be called only from GUI thread.
    pub fn get_existing_directory(
        title: Option<&str>,
        initial_path: Option<&Path>,
    ) -> Option<PathBuf> {
        if !Application::is_gui_thread() {
            panic!("FileDialog::get_existing_directory can be called only from GUI thread");
        }

        let caption = title
            .map_or_else(|| QString::null(), |v| QString::from_str(v))
            .unwrap();

        let dir = initial_path
            .map_or_else(
                || QString::null(),
                |v| QString::from_str(v.to_str().unwrap()),
            )
            .unwrap();

        QFileDialog::get_existing_directory(caption, dir, 0).map(|path| path.as_path_buf().unwrap())
    }

    /// Can be called only from GUI thread.
    pub fn get_save_file_name(
        title: Option<&str>,
        initial_path: Option<&Path>,
        filters: Option<&str>,
    ) -> Option<PathBuf> {
        if !Application::is_gui_thread() {
            panic!("FileDialog::get_save_file_name can be called only from GUI thread");
        }

        let caption = title
            .map_or_else(|| QString::null(), |v| QString::from_str(v))
            .unwrap();

        let dir = initial_path
            .map_or_else(
                || QString::null(),
                |v| QString::from_str(v.to_str().unwrap()),
            )
            .unwrap();

        let filters = filters
            .map_or_else(|| QString::null(), |v| QString::from_str(v))
            .unwrap();

        let mut selected_filter = QString::null().unwrap();

        QFileDialog::get_save_file_name(caption, dir, filters, &mut selected_filter, 0)
            .map(|path| path.as_path_buf().unwrap())
    }
}
