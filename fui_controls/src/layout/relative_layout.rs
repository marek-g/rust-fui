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
/// RelativeLayout is designed to work with Popup control
/// and it's dangerous (danger of panic) to use outside of it.
///
/// It works correctly only when:
/// 1. Covers whole window.
/// 2. References only controls on lower layouts.
///
/// The lower layout control is a control that:
/// - is not lying on the path from RelativeLayout to root
///   (that would cause borrow panic during layout phase)
/// - it's layout phase is calculated before the layout
///   phase of RelativeLayout (RelativeLayout refers to
///   calculated size of that control)
///
#[derive(TypedBuilder)]
pub struct RelativeLayout {
    #[builder(default = RelativePlacement::FullSize)]
    pub placement: RelativePlacement,

    /// Auto hide method.
    /// Defines rules when to call `auto_hide_request` callback.
    #[builder(default = PopupAutoHide::None)]
    pub auto_hide: PopupAutoHide,

    /// Called when auto hide is requested.
    #[builder(default = Callback::empty())]
    pub auto_hide_request: Callback<()>,

    /// RelativeLayout does not pass through events to controls below
    /// except the area covered by this list of controls
    #[builder(default = Vec::new())]
    pub uncovered_controls: Vec<Weak<RefCell<dyn ControlObject>>>,
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
                PopupAutoHide::ClickedOutside | PopupAutoHide::Menu => {
                    data.auto_hide_request.emit(())
                }
                _ => (),
            },

            ControlEvent::PointerMove { position } => match data.auto_hide {
                PopupAutoHide::Menu => {
                    if !position.is_inside(&self.relative_control_rect)
                        && !position.is_inside(&self.rect)
                    {
                        data.auto_hide_request.emit(())
                    }
                }
                _ => (),
            },

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut RelativeLayout,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) -> Size {
        // RelativeLayout is a special container.
        // It skips the measure phase of it's child here.
        // It delays it to the set_rect() phase, because
        // the space available to it's child may depend
        // on the position of the relative control
        // (the position of it is unknown here).
        self.rect = Rect::new(0.0f32, 0.0f32, size.width, size.height);
        size
    }

    fn set_rect(
        &mut self,
        data: &mut RelativeLayout,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        rect: Rect,
    ) {
        let children = control_context.get_children();

        let mut is_above = false;
        let mut is_left = false;
        self.relative_control_rect = Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);
        let available_size = match &data.placement {
            RelativePlacement::FullSize => Size::new(rect.width, rect.height),

            RelativePlacement::BelowOrAboveControl(relative_control) => {
                if let Some(relative_control) = relative_control.upgrade() {
                    self.relative_control_rect = relative_control.borrow().get_rect();

                    let height_above = self.relative_control_rect.y;
                    let height_below = rect.height
                        - (self.relative_control_rect.y + self.relative_control_rect.height);

                    if height_above > height_below {
                        is_above = true;
                        Size::new(self.relative_control_rect.width, height_above)
                    } else {
                        Size::new(self.relative_control_rect.width, height_below)
                    }
                } else {
                    Size::new(rect.width, rect.height)
                }
            }

            RelativePlacement::LeftOrRightControl(relative_control) => {
                if let Some(relative_control) = relative_control.upgrade() {
                    self.relative_control_rect = relative_control.borrow().get_rect();

                    let width_left = self.relative_control_rect.x;
                    let width_right = rect.width
                        - (self.relative_control_rect.x + self.relative_control_rect.width);

                    if width_left > width_right {
                        is_left = true;
                        Size::new(width_left, rect.height)
                    } else {
                        Size::new(width_right, rect.height)
                    }
                } else {
                    Size::new(rect.width, rect.height)
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

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(drawing_context, self.rect);
        }
    }

    fn hit_test(
        &self,
        data: &RelativeLayout,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        // If the point is inside child area
        // pass the check to the child.
        if point.is_inside(&self.rect) {
            let children = control_context.get_children();
            if let Some(ref content) = children.into_iter().next() {
                let c = content.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    let hit_control = c.hit_test(point);
                    if hit_control.is_some() {
                        return hit_control;
                    }
                }
            }
        }

        // If the point is over one of the `uncovered_controls`
        // return None, so the parent will hit test them
        // and they will receive event.
        for uncovered_control in &data.uncovered_controls {
            if let Some(uncovered_control) = uncovered_control.upgrade() {
                if point.is_inside(&uncovered_control.borrow().get_rect()) {
                    return None;
                }
            }
        }

        // Otherwise return self and block sending events outside.
        Some(control_context.get_self_rc())
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
