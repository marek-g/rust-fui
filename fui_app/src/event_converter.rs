pub fn convert_event(event: &winit::event::WindowEvent) -> Option<fui::InputEvent> {
    match event {
        winit::event::WindowEvent::CursorEntered { .. } => Some(fui::InputEvent::CursorEntered {}),

        winit::event::WindowEvent::CursorLeft { .. } => Some(fui::InputEvent::CursorLeft {}),

        winit::event::WindowEvent::CursorMoved { position, .. } => {
            Some(fui::InputEvent::CursorMoved {
                position: fui::Point::new(position.x as f32, position.y as f32),
            })
        }

        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            Some(fui::InputEvent::MouseInput {
                state: match state {
                    winit::event::ElementState::Pressed => fui::ElementState::Pressed,
                    winit::event::ElementState::Released => fui::ElementState::Released,
                },

                button: match button {
                    winit::event::MouseButton::Left => fui::MouseButton::Left,
                    winit::event::MouseButton::Right => fui::MouseButton::Right,
                    winit::event::MouseButton::Middle => fui::MouseButton::Middle,
                    winit::event::MouseButton::Other(other) => fui::MouseButton::Other(*other),
                },
            })
        }

        _ => None,
    }
}
