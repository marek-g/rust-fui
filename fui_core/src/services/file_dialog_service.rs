use std::path::{Path, PathBuf};

use async_trait::async_trait;

#[derive(Clone)]
pub struct FileFilter {
    pub name: String,
    pub filters: Vec<String>,
}

#[derive(Default)]
pub struct FileDialogData {
    pub title: Option<String>,
    pub initial_path: Option<PathBuf>,
    pub filters: Vec<FileFilter>,
}

impl FileDialogData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_initial_path<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.initial_path = Some(dir.as_ref().to_path_buf());
        self
    }

    pub fn with_filter(mut self, name: &str, filters: &[&str]) -> Self {
        self.filters.push(FileFilter {
            name: name.to_string(),
            filters: filters.iter().map(|e| e.to_string()).collect(),
        });
        self
    }

    pub fn with_filters(mut self, filters: Vec<FileFilter>) -> Self {
        self.filters = filters;
        self
    }
}

#[async_trait]
pub trait FileDialogService {
    async fn pick_file(&self, data: FileDialogData) -> Option<PathBuf>;
    async fn pick_files(&self, data: FileDialogData) -> Vec<PathBuf>;
    async fn pick_folder(&self, data: FileDialogData) -> Option<PathBuf>;
    async fn pick_save_file(&self, data: FileDialogData) -> Option<PathBuf>;
}
