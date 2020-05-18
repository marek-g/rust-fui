use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::transformation::*;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct Button {
    #[builder(default_code = "Callback::empty()")]
    pub clicked: Callback<()>,
}

impl Control for Button {
    fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(self,
            style.unwrap_or_else(|| {
                Box::new(DefaultButtonStyle::new(DefaultButtonStyleParams::builder().build()))
            }),
            context)
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
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            is_hover: Property::new(false),
            is_pressed: Property::new(false),
            is_focused: Property::new(false),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<Button> for DefaultButtonStyle {
    fn setup_dirty_watching(
        &mut self,
        _data: &mut Button,
        control: &Rc<RefCell<StyledControl<Button>>>,
    ) {
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_pressed.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_focused.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        data: &mut Button,
        context: &mut ControlContext,
        _resources: &mut dyn Resources,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_pressed.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &context, *position) {
                    data.clicked.emit(());
                }
                self.is_pressed.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &context, *position) {
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
        context: &mut ControlContext,
        resources: &mut dyn Resources,
        size: Size,
    ) {
        let children = context.get_children();
        let content_size = if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(resources, size);
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
        )
    }

    fn set_rect(&mut self, _data: &mut Button, context: &mut ControlContext, rect: Rect) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + 10.0f32,
            rect.y + 10.0f32,
            rect.width - 20.0f32,
            rect.height - 20.0f32,
        );

        let children = context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self, _context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Button, _context: &ControlContext, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        _data: &Button,
        context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

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

        let children = context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let mut vec2 = content.borrow_mut().to_primitives(resources);
            if self.is_pressed.get() {
                vec2.translate(PixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
        }

        vec
    }
}
