pub struct WindowOptions {
    pub title: String,
    pub icon: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

impl WindowOptions {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            icon: Vec::new(),
            width: 800,
            height: 600,
            visible: true,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_icon(mut self, icon: Vec<u8>) -> Self {
        self.icon = icon;
        self
    }

    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}
