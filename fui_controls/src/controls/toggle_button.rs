use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::transformation::*;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct ToggleButton {
    #[builder(default = Property::new(false))]
    pub is_checked: Property<bool>,
}

impl ToggleButton {
    pub fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(self,
            style.unwrap_or_else(|| {
                Box::new(DefaultToggleButtonStyle::new(DefaultToggleButtonStyleParams::builder().build()))
            }),
            context)
    }
}

//
// Default ToggleButton Style
//

#[derive(TypedBuilder)]
pub struct DefaultToggleButtonStyleParams {}

pub struct DefaultToggleButtonStyle {
    rect: Rect,
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultToggleButtonStyle {
    pub fn new(_params: DefaultToggleButtonStyleParams) -> Self {
        DefaultToggleButtonStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ToggleButton> for DefaultToggleButtonStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut ToggleButton,
        control: &Rc<RefCell<StyledControl<ToggleButton>>>,
    ) {
        self.event_subscriptions
            .push(data.is_checked.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_tapped.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_focused.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        context: &mut ControlContext,
        _resources: &mut dyn Resources,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &context, *position) {
                    data.is_checked.change(|val| !val);
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &context, *position) {
                    self.is_tapped.set(true);
                } else {
                    self.is_tapped.set(false);
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
        _data: &mut ToggleButton,
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

    fn set_rect(&mut self, _data: &mut ToggleButton, context: &mut ControlContext, rect: Rect) {
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

    fn hit_test(&self, _data: &ToggleButton, _context: &ControlContext, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &ToggleButton,
        context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let is_pressed = if self.is_tapped.get() {
            !data.is_checked.get()
        } else {
            data.is_checked.get()
        };

        default_theme::button(
            &mut vec,
            x,
            y,
            width,
            height,
            is_pressed,
            self.is_hover.get(),
            self.is_focused.get(),
        );

        let children = context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let mut vec2 = content.borrow_mut().to_primitives(resources);
            if is_pressed {
                vec2.translate(PixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
        }

        vec
    }
}


//
// Tab ToggleButton Style
// (cannot be unpressed).
//

#[derive(TypedBuilder)]
pub struct TabToggleButtonStyleParams {}

pub struct TabToggleButtonStyle {
    rect: Rect,
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
    event_subscriptions: Vec<EventSubscription>,
}

impl TabToggleButtonStyle {
    pub fn new(_params: TabToggleButtonStyleParams) -> Self {
        TabToggleButtonStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ToggleButton> for TabToggleButtonStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut ToggleButton,
        control: &Rc<RefCell<StyledControl<ToggleButton>>>,
    ) {
        self.event_subscriptions
            .push(data.is_checked.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_tapped.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_focused.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        context: &mut ControlContext,
        _resources: &mut dyn Resources,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &context, *position) {
                    data.is_checked.set(true);
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &context, *position) {
                    self.is_tapped.set(true);
                } else {
                    self.is_tapped.set(false);
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
        _data: &mut ToggleButton,
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

    fn set_rect(&mut self, _data: &mut ToggleButton, context: &mut ControlContext, rect: Rect) {
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

    fn hit_test(&self, _data: &ToggleButton, _context: &ControlContext, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &ToggleButton,
        context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let is_pressed = if self.is_tapped.get() {
            true
        } else {
            data.is_checked.get()
        };

        default_theme::button(
            &mut vec,
            x,
            y,
            width,
            height,
            is_pressed,
            self.is_hover.get(),
            self.is_focused.get(),
        );

        let children = context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let mut vec2 = content.borrow_mut().to_primitives(resources);
            if is_pressed {
                vec2.translate(PixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
        }

        vec
    }
}
