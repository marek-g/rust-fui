use std::cell::RefCell;
use std::rc::Rc;

use drawing::clipping::Clipping;
use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize};
use fui_core::*;
use typed_builder::TypedBuilder;

#[derive(PartialEq, Clone, Default)]
pub struct ViewportInfo {
    pub content_width: f32,
    pub content_height: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

#[derive(TypedBuilder)]
pub struct ScrollArea {
    #[builder(default = Property::new(0.0f32))]
    pub offset_x: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub offset_y: Property<f32>,

    #[builder(default = Property::new(ViewportInfo::default()))]
    pub viewport_info: Property<ViewportInfo>,
}

impl ScrollArea {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultScrollAreaStyle::new(
                    DefaultScrollAreaStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default ScrollArea Style
//

#[derive(TypedBuilder)]
pub struct DefaultScrollAreaStyleParams {}

pub struct DefaultScrollAreaStyle {
    content_size: Size,
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultScrollAreaStyle {
    pub fn new(_params: DefaultScrollAreaStyleParams) -> Self {
        DefaultScrollAreaStyle {
            content_size: Size::new(0.0f32, 0.0f32),
            event_subscriptions: Vec::new(),
        }
    }

    fn update_properties(&self, data: &mut ScrollArea, rect: Rect) {
        data.viewport_info.set(ViewportInfo {
            content_width: self.content_size.width,
            content_height: self.content_size.height,
            viewport_width: rect.width,
            viewport_height: rect.height,
        });

        let max_offset_x = (self.content_size.width - rect.width).max(0.0f32);
        let max_offset_y = (self.content_size.height - rect.height).max(0.0f32);
        if data.offset_x.get() > max_offset_x {
            data.offset_x.set(max_offset_x)
        }
        if data.offset_y.get() > max_offset_y {
            data.offset_y.set(max_offset_y)
        }
    }
}

impl Style<ScrollArea> for DefaultScrollAreaStyle {
    fn setup(&mut self, data: &mut ScrollArea, control_context: &mut ControlContext) {
        self.event_subscriptions
            .push(data.offset_x.dirty_watching(&control_context.get_self_rc()));
        self.event_subscriptions
            .push(data.offset_y.dirty_watching(&control_context.get_self_rc()));
    }

    fn handle_event(
        &mut self,
        _data: &mut ScrollArea,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        _data: &mut ScrollArea,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) -> Size {
        let children = control_context.get_children();
        self.content_size = if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(drawing_context, size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };

        Size::new(
            self.content_size.width.min(size.width),
            self.content_size.height.min(size.height),
        )
    }

    fn set_rect(
        &mut self,
        data: &mut ScrollArea,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        rect: Rect,
    ) {
        self.update_properties(data, rect);

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let child_rect = Rect::new(
                rect.x - data.offset_x.get().round(),
                rect.y - data.offset_y.get().round(),
                rect.width + data.offset_x.get().round(),
                rect.height + data.offset_y.get().round(),
            );
            content.borrow_mut().set_rect(drawing_context, child_rect);
        }
    }

    fn hit_test(
        &self,
        _data: &ScrollArea,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if point.is_inside(&control_context.get_rect()) {
            let children = control_context.get_children();
            if let Some(ref content) = children.into_iter().next() {
                let c = content.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    if let Some(hit_control) = c.hit_test(point) {
                        return Some(hit_control);
                    }
                }
            }
            None
        } else {
            None
        }
    }

    fn to_primitives(
        &self,
        _data: &ScrollArea,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let rect = control_context.get_rect();
        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let (vec2, mut overlay2) = content.borrow_mut().to_primitives(drawing_context);

            let mut vec2 = vec2.clip(PixelRect::new(
                PixelPoint::new(x, y),
                PixelSize::new(width, height),
            ));

            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}
