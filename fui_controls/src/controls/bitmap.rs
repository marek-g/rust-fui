use std::cell::RefCell;
use std::rc::Rc;

use crate::Alignment;
use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize};
use fui_core::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Bitmap {
    pub texture_id: Property<i32>,
}

impl Bitmap {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultBitmapStyle::new(
                    DefaultBitmapStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default Bitmap Style
//

#[derive(TypedBuilder)]
pub struct DefaultBitmapStyleParams {}

pub struct DefaultBitmapStyle {
    rect: Rect,
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultBitmapStyle {
    pub fn new(_params: DefaultBitmapStyleParams) -> Self {
        DefaultBitmapStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<Bitmap> for DefaultBitmapStyle {
    fn setup(&mut self, data: &mut Bitmap, control_context: &mut ControlContext) {
        self.event_subscriptions.push(
            data.texture_id
                .dirty_watching(&control_context.get_self_rc()),
        );
    }

    fn handle_event(
        &mut self,
        _data: &mut Bitmap,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Bitmap,
        _control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        _size: Size,
    ) {
        self.rect = if let Ok(texture_size) = drawing_context
            .get_resources()
            .get_texture_size(data.texture_id.get())
        {
            Rect::new(0.0f32, 0.0f32, texture_size.0 as f32, texture_size.1 as f32)
        } else {
            Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32)
        }
    }

    fn set_rect(&mut self, _data: &mut Bitmap, control_context: &mut ControlContext, rect: Rect) {
        let map = control_context.get_attached_values();
        Alignment::apply(
            &mut self.rect,
            rect,
            &map,
            Alignment::Start,
            Alignment::Start,
        );
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &Bitmap,
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
        data: &Bitmap,
        _control_context: &ControlContext,
        _drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();

        if self.rect.width > 0.0f32 && self.rect.height > 0.0f32 {
            vec.push(Primitive::Image {
                resource_key: data.texture_id.get(),
                rect: PixelRect::new(
                    PixelPoint::new(self.rect.x, self.rect.y),
                    PixelSize::new(self.rect.width, self.rect.height),
                ),
                uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
            });
        }

        (vec, Vec::new())
    }
}
