use std::cell::RefCell;
use std::rc::Rc;

use fui_core::*;
use fui_drawing::prelude::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct Button {
    #[builder(default = Callback::empty())]
    pub clicked: Callback<()>,
}

impl Button {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultButtonStyle::new(
                    DefaultButtonStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default Button Style
//

#[derive(TypedBuilder)]
pub struct DefaultButtonStyleParams {}

pub struct DefaultButtonStyle {
    is_hover: Property<bool>,
    is_pressed: Property<bool>,
    is_focused: Property<bool>,
}

impl DefaultButtonStyle {
    pub fn new(_params: DefaultButtonStyleParams) -> Self {
        DefaultButtonStyle {
            is_hover: Property::new(false),
            is_pressed: Property::new(false),
            is_focused: Property::new(false),
        }
    }
}

impl Style<Button> for DefaultButtonStyle {
    fn setup(&mut self, _data: &mut Button, control_context: &mut ControlContext) {
        control_context.dirty_watch_property(&self.is_hover);
        control_context.dirty_watch_property(&self.is_pressed);
        control_context.dirty_watch_property(&self.is_focused);
    }

    fn handle_event(
        &mut self,
        data: &mut Button,
        control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_pressed.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let Some(hit_control) = self.hit_test(&data, &control_context, *position) {
                    if Rc::ptr_eq(&hit_control, &control_context.get_self_rc()) {
                        data.clicked.emit(());
                    }
                }
                self.is_pressed.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                match self.hit_test(&data, &control_context, *position) {
                    Some(hit_control) => {
                        if Rc::ptr_eq(&hit_control, &control_context.get_self_rc()) {
                            self.is_pressed.set(true);
                        } else {
                            self.is_pressed.set(false);
                        }
                    }
                    _ => {
                        self.is_pressed.set(false);
                    }
                }
            }

            ControlEvent::HoverChange(value) => {
                self.is_hover.set(value);
            }

            ControlEvent::FocusChange(value) => {
                self.is_focused.set(value);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut Button,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        mut size: Size,
    ) -> Size {
        if size.width.is_finite() {
            size.width = 0.0f32.max(size.width - 20.0f32);
        }
        if size.height.is_finite() {
            size.height = 0.0f32.max(size.height - 20.0f32);
        }

        let children = control_context.get_children();
        let content_size = match children.into_iter().next() {
            Some(ref content) => {
                content.borrow_mut().measure(drawing_context, size);
                let rect = content.borrow().get_rect();
                Size::new(rect.width, rect.height)
            }
            _ => Size::new(0f32, 0f32),
        };

        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_rect(
        &mut self,
        _data: &mut Button,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    ) {
        let content_rect = Rect::new(
            rect.x + 10.0f32,
            rect.y + 10.0f32,
            rect.width - 20.0f32,
            rect.height - 20.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(drawing_context, content_rect);
        }
    }

    fn hit_test(
        &self,
        _data: &Button,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        _data: &Button,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let rect = control_context.get_rect();

        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        default_theme::button(
            &mut drawing_context.display,
            x,
            y,
            width,
            height,
            self.is_pressed.get(),
            self.is_hover.get(),
            self.is_focused.get(),
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            if self.is_pressed.get() {
                drawing_context.display.save();
                drawing_context.display.translate(1.0, 1.0);
            }
            content.borrow_mut().draw(drawing_context);
            if self.is_pressed.get() {
                drawing_context.display.restore();
            }
        }
    }
}
