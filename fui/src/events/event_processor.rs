extern crate winit;

use std::cell::RefCell;
use std::rc::Rc;

use control::ControlObject;
use events::*;
use ViewData;
use RootView;

#[derive(Clone, Debug, PartialEq)]
pub enum ControlEvent {
    TapDown { position: (f32, f32) },
    TapUp { position: (f32, f32) }
}

pub struct EventProcessor {
    gesture_detector: GestureDetector,
    mouse_pos: (f32, f32),
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {
            gesture_detector: GestureDetector::new(),
            mouse_pos: (0f32, 0f32),
        }
    }

    pub fn handle_event(&mut self, root_view: &mut RootView, event: &winit::Event) {
        self.handle_hover(root_view, event);

        self.gesture_detector.handle_event(event).map(|ev| match ev {
            Gesture::TapUp { position, captured_position } => {
                self.dispatch_event_by_hit_target(&root_view.view_data.root_control, captured_position, ControlEvent::TapUp { position: position });
            },
            _ => ()
        });
    }

    fn handle_hover(&mut self, root_view: &mut RootView, event: &winit::Event) {
        if let ::winit::Event::WindowEvent { ref event, .. } = event {
            match event {
                winit::WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_pos = (position.0 as f32, position.1 as f32);
                },
                _ => ()
            }
        }
    }

    fn dispatch_event_by_hit_target(&mut self, root_control: &Rc<RefCell<ControlObject>>, hit_target: (f32, f32), event: ControlEvent) {
        let is_hit_test_visible = root_control.borrow().is_hit_test_visible();
        if is_hit_test_visible {
            let rect = root_control.borrow().get_rect();
            if hit_target.0 >= rect.x && hit_target.0 <= rect.x + rect.width &&
                hit_target.1 >= rect.y && hit_target.1 <= rect.y + rect.height {
                root_control.borrow_mut().handle_event(event);
                return;
            }
        }

        for child in root_control.borrow_mut().get_children() {
            self.dispatch_event_by_hit_target(&child, hit_target, event.clone());
        }
    }
}