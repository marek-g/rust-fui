use std::cell::RefCell;
use std::rc::Rc;

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
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultBitmapStyle {
    pub fn new(_params: DefaultBitmapStyleParams) -> Self {
        DefaultBitmapStyle {
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
    ) -> Size {
        if let Ok(texture_size) = drawing_context
            .get_resources()
            .get_texture_size(data.texture_id.get())
        {
            Size::new(texture_size.0 as f32, texture_size.1 as f32)
        } else {
            Size::new(0.0f32, 0.0f32)
        }
    }

    fn set_rect(&mut self, _data: &mut Bitmap, _control_context: &mut ControlContext, _rect: Rect) {
    }

    fn hit_test(
        &self,
        _data: &Bitmap,
        control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&control_context.get_rect()) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &Bitmap,
        control_context: &ControlContext,
        _drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();

        let rect = control_context.get_rect();
        if rect.width > 0.0f32 && rect.height > 0.0f32 {
            vec.push(Primitive::Image {
                resource_key: data.texture_id.get(),
                rect: PixelRect::new(
                    PixelPoint::new(rect.x, rect.y),
                    PixelSize::new(rect.width, rect.height),
                ),
                uv: [0.0f32, 0.0f32, 1.0f32, 1.0f32],
            });
        }

        (vec, Vec::new())
    }
}
