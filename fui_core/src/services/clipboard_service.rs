pub enum ClipboardMode {
    Clipboard,
    Selection,
    FindBuffer,
}

pub trait ClipboardService {
    fn set_text(&self, text: &str, mode: ClipboardMode);
    fn get_text(&self, mode: ClipboardMode) -> Option<String>;
}
