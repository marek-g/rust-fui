use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::transformation::*;
use drawing::units::PixelPoint;
use fui_core::*;
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
    ) -> Rc<RefCell<StyledControl<Self>>> {
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
    rect: Rect,
    is_hover: Property<bool>,
    is_pressed: Property<bool>,
    is_focused: Property<bool>,
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultButtonStyle {
    pub fn new(_params: DefaultButtonStyleParams) -> Self {
        DefaultButtonStyle {
            rect: Rect::empty(),
            is_hover: Property::new(false),
            is_pressed: Property::new(false),
            is_focused: Property::new(false),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<Button> for DefaultButtonStyle {
    fn setup(&mut self, _data: &mut Button, control_context: &mut ControlContext) {
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(&control_context.get_self_rc()));
        self.event_subscriptions.push(
            self.is_pressed
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions.push(
            self.is_focused
                .dirty_watching(&control_context.get_self_rc()),
        );
    }

    fn handle_event(
        &mut self,
        data: &mut Button,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_pressed.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    data.clicked.emit(());
                }
                self.is_pressed.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    self.is_pressed.set(true);
                } else {
                    self.is_pressed.set(false);
                }
            }

            ControlEvent::HoverEnter => {
                self.is_hover.set(true);
            }

            ControlEvent::HoverLeave => {
                self.is_hover.set(false);
            }

            ControlEvent::FocusEnter => {
                self.is_focused.set(true);
            }

            ControlEvent::FocusLeave => {
                self.is_focused.set(false);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut Button,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        mut size: Size,
    ) {
        if size.width.is_finite() {
            size.width = 0.0f32.max(size.width - 20.0f32);
        }
        if size.height.is_finite() {
            size.height = 0.0f32.max(size.height - 20.0f32);
        }

        let children = control_context.get_children();
        let content_size = if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(drawing_context, size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };

        self.rect = Rect::new(
            0.0f32,
            0.0f32,
            content_size.width + 20.0f32,
            content_size.height + 20.0f32,
        );
    }

    fn set_rect(&mut self, _data: &mut Button, control_context: &mut ControlContext, rect: Rect) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + 10.0f32,
            rect.y + 10.0f32,
            rect.width - 20.0f32,
            rect.height - 20.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &Button,
        _control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        _data: &Button,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        default_theme::button(
            &mut vec,
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
            let (mut vec2, mut overlay2) = content.borrow_mut().to_primitives(drawing_context);
            if self.is_pressed.get() {
                vec2.translate(PixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}
