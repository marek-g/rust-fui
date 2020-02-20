use std::cell::RefCell;
use std::rc::Rc;

use drawing::clipping::Clipping;
use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
use typed_builder::TypedBuilder;

use crate::style::*;

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

impl View for ScrollArea {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        Control::new(self, ScrollAreaDefaultStyle::new(), context)
    }
}

//
// ScrollViewer Default Style
//

pub struct ScrollAreaDefaultStyle {
    rect: Rect,
    content_size: Size,
    event_subscriptions: Vec<EventSubscription>,
}

impl ScrollAreaDefaultStyle {
    pub fn new() -> Self {
        ScrollAreaDefaultStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            content_size: Size::new(0.0f32, 0.0f32),
            event_subscriptions: Vec::new(),
        }
    }

    fn update_properties(&self, data: &mut ScrollArea) {
        data.viewport_info.set(ViewportInfo {
            content_width: self.content_size.width,
            content_height: self.content_size.height,
            viewport_width: self.rect.width,
            viewport_height: self.rect.height,
        });

        let max_offset_x = (self.content_size.width - self.rect.width).max(0.0f32);
        let max_offset_y = (self.content_size.height - self.rect.height).max(0.0f32);
        if data.offset_x.get() > max_offset_x {
            data.offset_x.set(max_offset_x)
        }
        if data.offset_y.get() > max_offset_y {
            data.offset_y.set(max_offset_y)
        }
    }
}

impl Style<ScrollArea> for ScrollAreaDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut ScrollArea,
        control: &Rc<RefCell<Control<ScrollArea>>>,
    ) {
        self.event_subscriptions
            .push(data.offset_x.dirty_watching(control));
        self.event_subscriptions
            .push(data.offset_y.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        _data: &mut ScrollArea,
        _context: &mut ControlContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        _data: &mut ScrollArea,
        context: &mut ControlContext,
        resources: &mut dyn Resources,
        size: Size,
    ) {
        let children = context.get_children();
        self.content_size = if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(resources, size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };

        self.rect = Rect::new(
            0.0f32,
            0.0f32,
            self.content_size.width.min(size.width),
            self.content_size.height.min(size.height),
        );
    }

    fn set_rect(&mut self, data: &mut ScrollArea, context: &mut ControlContext, rect: Rect) {
        self.rect = rect;

        self.update_properties(data);

        let children = context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let child_rect = Rect::new(
                rect.x - data.offset_x.get().round(),
                rect.y - data.offset_y.get().round(),
                rect.width + data.offset_x.get().round(),
                rect.height + data.offset_y.get().round(),
            );
            content.borrow_mut().set_rect(child_rect);
        }
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ScrollArea,
        context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            let children = context.get_children();
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
            HitTestResult::Nothing
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &ScrollArea,
        context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let children = context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let vec2 = content.borrow_mut().to_primitives(resources);

            let mut vec2 = vec2.clip(PixelRect::new(
                PixelPoint::new(x, y),
                PixelSize::new(width, height),
            ));

            vec.append(&mut vec2);
        }

        vec
    }
}
