use async_trait::async_trait;
use fui_core::{FileDialogData, FileDialogService};
use std::path::PathBuf;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;

pub struct AppFileDialog;

#[async_trait]
impl FileDialogService for AppFileDialog {
    async fn pick_file(&self, data: FileDialogData) -> Option<PathBuf> {
        let (sender, receiver) = oneshot::channel::<Option<PathBuf>>();

        fui_system::Application::post_func({
            move || {
                let result = fui_system::FileDialog::get_open_file_name(
                    data.title.as_deref(),
                    data.initial_path.as_deref(),
                    filters_to_string(data.filters).as_deref(),
                );
                sender.send(result).unwrap();
            }
        });

        receiver.await.unwrap()
    }

    async fn pick_files(&self, data: FileDialogData) -> Vec<PathBuf> {
        let (sender, receiver) = oneshot::channel::<Vec<PathBuf>>();

        fui_system::Application::post_func({
            move || {
                let result = fui_system::FileDialog::get_open_file_names(
                    data.title.as_deref(),
                    data.initial_path.as_deref(),
                    filters_to_string(data.filters).as_deref(),
                );
                sender.send(result).unwrap();
            }
        });

        receiver.await.unwrap()
    }

    async fn pick_folder(&self, data: FileDialogData) -> Option<PathBuf> {
        let (sender, receiver) = oneshot::channel::<Option<PathBuf>>();

        fui_system::Application::post_func({
            move || {
                let result = fui_system::FileDialog::get_existing_directory(
                    data.title.as_deref(),
                    data.initial_path.as_deref(),
                );
                sender.send(result).unwrap();
            }
        });

        receiver.await.unwrap()
    }

    async fn pick_save_file(&self, data: FileDialogData) -> Option<PathBuf> {
        let (sender, receiver) = oneshot::channel::<Option<PathBuf>>();

        fui_system::Application::post_func({
            move || {
                let result = fui_system::FileDialog::get_save_file_name(
                    data.title.as_deref(),
                    data.initial_path.as_deref(),
                    filters_to_string(data.filters).as_deref(),
                );
                sender.send(result).unwrap();
            }
        });

        receiver.await.unwrap()
    }
}

fn filters_to_string(filters: Vec<fui_core::Filter>) -> Option<String> {
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
