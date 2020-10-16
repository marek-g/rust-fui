use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::PopupAutoHide;
use drawing::primitive::Primitive;
use fui_core::*;
use typed_builder::TypedBuilder;

pub enum RelativePlacement {
    FullSize,
    BelowOrAboveControl(Weak<RefCell<dyn ControlObject>>),
    LeftOrRightControl(Weak<RefCell<dyn ControlObject>>),
}

///
/// Warning!
///
/// RelativeLayout is designed to work with Popup control.
///
/// It works correctly only when:
/// 1. Placed as a top control (covers whole window).
/// 2. References to controls on lower layouts.
///
/// Referencing controls placed on the same layout
/// can cause panics because of recursive borrowing
/// controls during layout phase.
///
#[derive(TypedBuilder)]
pub struct RelativeLayout {
    #[builder(default = RelativePlacement::FullSize)]
    pub placement: RelativePlacement,

    #[builder(default = Callback::empty())]
    pub close: Callback<()>,

    #[builder(default = PopupAutoHide::None)]
    pub auto_hide: PopupAutoHide,
}

impl RelativeLayout {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultRelativeLayoutStyle::new(
                    DefaultRelativeLayoutStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default RelativeLayout Style
//

#[derive(TypedBuilder)]
pub struct DefaultRelativeLayoutStyleParams {}

pub struct DefaultRelativeLayoutStyle {
    rect: Rect,
    relative_control_rect: Rect,
}

impl DefaultRelativeLayoutStyle {
    pub fn new(_params: DefaultRelativeLayoutStyleParams) -> Self {
        DefaultRelativeLayoutStyle {
            rect: Rect::empty(),
            relative_control_rect: Rect::empty(),
        }
    }
}

impl Style<RelativeLayout> for DefaultRelativeLayoutStyle {
    fn setup(&mut self, _data: &mut RelativeLayout, _control_context: &mut ControlContext) {}

    fn handle_event(
        &mut self,
        data: &mut RelativeLayout,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => match data.auto_hide {
                PopupAutoHide::ClickedOutside | PopupAutoHide::Menu => data.close.emit(()),
                _ => (),
            },

            ControlEvent::PointerMove { position } => match data.auto_hide {
                PopupAutoHide::Menu => {
                    if !position.is_inside(&self.relative_control_rect)
                        && !position.is_inside(&self.rect)
                    {
                        data.close.emit(())
                    }
                }
                _ => (),
            },

            _ => (),
        }
    }

    fn measure(
        &mut self,
        data: &mut RelativeLayout,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) -> Size {
        let children = control_context.get_children();

        let mut is_above = false;
        let mut is_left = false;
        self.relative_control_rect = Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);
        let available_size = match &data.placement {
            RelativePlacement::FullSize => size,

            RelativePlacement::BelowOrAboveControl(relative_control) => {
                if let Some(relative_control) = relative_control.upgrade() {
                    self.relative_control_rect = relative_control.borrow().get_rect();

                    let height_above = self.relative_control_rect.y;
                    let height_below = size.height
                        - (self.relative_control_rect.y + self.relative_control_rect.height);

                    if height_above > height_below {
                        is_above = true;
                        Size::new(self.relative_control_rect.width, height_above)
                    } else {
                        Size::new(self.relative_control_rect.width, height_below)
                    }
                } else {
                    size
                }
            }

            RelativePlacement::LeftOrRightControl(relative_control) => {
                if let Some(relative_control) = relative_control.upgrade() {
                    self.relative_control_rect = relative_control.borrow().get_rect();

                    let width_left = self.relative_control_rect.x;
                    let width_right = size.width
                        - (self.relative_control_rect.x + self.relative_control_rect.width);

                    if width_left > width_right {
                        is_left = true;
                        Size::new(width_left, size.height)
                    } else {
                        Size::new(width_right, size.height)
                    }
                } else {
                    size
                }
            }
        };

        let content_size = if let Some(ref content) = children.into_iter().next() {
            content
                .borrow_mut()
                .measure(drawing_context, available_size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };

        self.rect = match &data.placement {
            RelativePlacement::FullSize => {
                Rect::new(0.0f32, 0.0f32, available_size.width, available_size.height)
            }

            RelativePlacement::BelowOrAboveControl(_) => {
                if is_above {
                    Rect::new(
                        self.relative_control_rect.x,
                        self.relative_control_rect.y - content_size.height,
                        content_size.width.max(available_size.width),
                        content_size.height,
                    )
                } else {
                    Rect::new(
                        self.relative_control_rect.x,
                        self.relative_control_rect.y + self.relative_control_rect.height,
                        content_size.width.max(available_size.width),
                        content_size.height,
                    )
                }
            }

            RelativePlacement::LeftOrRightControl(_) => {
                let pos_y = if content_size.height
                    <= available_size.height - self.relative_control_rect.y
                {
                    self.relative_control_rect.y
                } else {
                    (self.relative_control_rect.y + self.relative_control_rect.height
                        - content_size.height)
                        .max(0.0f32)
                };
                if is_left {
                    Rect::new(
                        self.relative_control_rect.x - content_size.width.min(available_size.width),
                        pos_y,
                        content_size.width.min(available_size.width),
                        content_size.height,
                    )
                } else {
                    Rect::new(
                        self.relative_control_rect.x + self.relative_control_rect.width,
                        pos_y,
                        content_size.width.min(available_size.width),
                        content_size.height,
                    )
                }
            }
        };

        Size::new(self.rect.width, self.rect.height)
    }

    fn set_rect(
        &mut self,
        _data: &mut RelativeLayout,
        control_context: &mut ControlContext,
        _rect: Rect,
    ) {
        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(self.rect);
        }
    }

    fn hit_test(
        &self,
        _data: &RelativeLayout,
        control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            let children = control_context.get_children();
            if let Some(ref content) = children.into_iter().next() {
                let c = content.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    let child_hit_test = c.hit_test(point);
                    match child_hit_test {
                        HitTestResult::Current => return HitTestResult::Child(content.clone()),
                        HitTestResult::Child(..) => return child_hit_test,
                        HitTestResult::Nothing => (),
                    }
                }
            }
            HitTestResult::Current
        } else {
            HitTestResult::Current
        }
    }

    fn to_primitives(
        &self,
        _data: &RelativeLayout,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow().to_primitives(drawing_context)
        } else {
            (Vec::new(), Vec::new())
        }
    }
}
