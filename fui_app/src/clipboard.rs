use std::sync::mpsc;

use fui_core::ClipboardService;

pub struct Clipboard {}

impl ClipboardService for Clipboard {
    fn set_text(&self, text: &str, mode: fui_core::ClipboardMode) {
        let (sender, receiver) = mpsc::channel();
        let text = text.to_string();

        windowing_qt::Application::post_func(move || {
            let clipboard = windowing_qt::Application::clipboard();
            let mode = convert_mode(mode);
            clipboard.set_text(&text, mode).unwrap();
            sender.send(()).unwrap();
        });

        receiver.recv().unwrap();
    }

    fn get_text(&self, mode: fui_core::ClipboardMode) -> Option<String> {
        let (sender, receiver) = mpsc::channel();

        windowing_qt::Application::post_func(move || {
            let clipboard = windowing_qt::Application::clipboard();
            let mode = convert_mode(mode);
            let result = clipboard.get_text(mode);
            sender.send(result).unwrap();
        });

        receiver.recv().unwrap()
    }
}

fn convert_mode(mode: fui_core::ClipboardMode) -> windowing_qt::ClipboardMode {
    let mode = match mode {
        fui_core::ClipboardMode::Clipboard => windowing_qt::ClipboardMode::Clipboard,
        fui_core::ClipboardMode::Selection => windowing_qt::ClipboardMode::Selection,
        fui_core::ClipboardMode::FindBuffer => windowing_qt::ClipboardMode::FindBuffer,
    };
    mode
}
