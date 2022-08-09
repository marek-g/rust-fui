use std::future::Future;
use std::path::{Path, PathBuf};
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;

pub struct Filter {
    pub name: String,
    pub filters: Vec<String>,
}

#[derive(Default)]
pub struct FileDialog {
    pub title: Option<String>,
    pub initial_path: Option<PathBuf>,
    pub filters: Vec<Filter>,
}

impl FileDialog {
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
        self.filters.push(Filter {
            name: name.to_string(),
            filters: filters.iter().map(|e| e.to_string()).collect(),
        });
        self
    }

    pub async fn pick_file(self) -> Option<PathBuf> {
        self.pick_file_internal().await.unwrap()
    }

    pub async fn pick_files(self) -> Vec<PathBuf> {
        self.pick_files_internal().await.unwrap()
    }

    pub async fn pick_folder(self) -> Option<PathBuf> {
        self.pick_folder_internal().await.unwrap()
    }

    pub async fn pick_save_file(self) -> Option<PathBuf> {
        self.pick_save_file_internal().await.unwrap()
    }

    fn pick_file_internal(self) -> impl Future<Output = Result<Option<PathBuf>, RecvError>> {
        let (sender, receiver) = oneshot::channel::<Option<PathBuf>>();

        fui_system::Application::post_func({
            move || {
                let result = fui_system::FileDialog::get_open_file_name(
                    self.title.as_deref(),
                    self.initial_path.as_deref(),
                    filters_to_string(self.filters).as_deref(),
                );
                sender.send(result).unwrap();
            }
        });

        receiver
    }

    fn pick_files_internal(self) -> impl Future<Output = Result<Vec<PathBuf>, RecvError>> {
        let (sender, receiver) = oneshot::channel::<Vec<PathBuf>>();

        fui_system::Application::post_func({
            move || {
                let result = fui_system::FileDialog::get_open_file_names(
                    self.title.as_deref(),
                    self.initial_path.as_deref(),
                    filters_to_string(self.filters).as_deref(),
                );
                sender.send(result).unwrap();
            }
        });

        receiver
    }

    fn pick_folder_internal(self) -> impl Future<Output = Result<Option<PathBuf>, RecvError>> {
        let (sender, receiver) = oneshot::channel::<Option<PathBuf>>();

        fui_system::Application::post_func({
            move || {
                let result = fui_system::FileDialog::get_existing_directory(
                    self.title.as_deref(),
                    self.initial_path.as_deref(),
                );
                sender.send(result).unwrap();
            }
        });

        receiver
    }

    fn pick_save_file_internal(self) -> impl Future<Output = Result<Option<PathBuf>, RecvError>> {
        let (sender, receiver) = oneshot::channel::<Option<PathBuf>>();

        fui_system::Application::post_func({
            move || {
                let result = fui_system::FileDialog::get_save_file_name(
                    self.title.as_deref(),
                    self.initial_path.as_deref(),
                    filters_to_string(self.filters).as_deref(),
                );
                sender.send(result).unwrap();
            }
        });

        receiver
    }
}

fn filters_to_string(filters: Vec<Filter>) -> Option<String> {
    if filters.is_empty() {
        None
    } else {
        Some(
            filters
                .into_iter()
                .map(|f| format!("{} ({})", f.name, f.filters.join(" ")))
                .collect::<Vec<String>>()
                .join(";;"),
        )
    }
}
