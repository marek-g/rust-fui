extern crate winit;

use control::ControlObject;

pub struct EventProcessor {

}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor { }
    }

    pub fn handle_event(&mut self, root_control: &mut Box<ControlObject>, event: &winit::Event) {
        if self.handle_mouse_event(root_control, event) { return };

        root_control.handle_event(&event);
    }

    fn handle_mouse_event(&mut self, root_control: &mut Box<ControlObject>, event: &winit::Event) -> bool {
        false
    }
}