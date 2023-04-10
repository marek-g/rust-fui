use fui_system::{TranslucentEffect, WindowFrameType};

#[derive(Clone)]
pub struct WindowOptions {
    pub title: String,
    pub icon: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub stay_on_top: bool,
    pub transparent_for_input: bool,
    pub translucent_effect: TranslucentEffect,
    pub frame_type: WindowFrameType,
    pub visible: bool,
}

impl WindowOptions {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            icon: Vec::new(),
            width: 800,
            height: 600,
            stay_on_top: false,
            transparent_for_input: false,
            translucent_effect: TranslucentEffect::None,
            frame_type: WindowFrameType::Normal,
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

    pub fn with_stay_on_top(mut self, stay_on_top: bool) -> Self {
        self.stay_on_top = stay_on_top;
        self
    }

    pub fn with_transparent_for_input(mut self, transparent_for_input: bool) -> Self {
        self.transparent_for_input = transparent_for_input;
        self
    }

    pub fn with_translucent_background(mut self, translucent_effect: TranslucentEffect) -> Self {
        self.translucent_effect = translucent_effect;
        self
    }

    pub fn with_frame_type(mut self, frame_type: WindowFrameType) -> Self {
        self.frame_type = frame_type;
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}
