use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct Bitmap {
    pub texture_id: Property<i32>,
}

impl View for Bitmap {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(self, BitmapDefaultStyle::new(), context)
    }
}

//
// Bitmap Default Style
//

pub struct BitmapDefaultStyle {
    rect: Rect,
    event_subscriptions: Vec<EventSubscription>,
}

impl BitmapDefaultStyle {
    pub fn new() -> BitmapDefaultStyle {
        BitmapDefaultStyle {
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

impl Style<Bitmap> for BitmapDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut Bitmap,
        control: &Rc<RefCell<StyledControl<Bitmap>>>,
    ) {
        self.event_subscriptions
            .push(data.texture_id.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        _data: &mut Bitmap,
        _context: &mut ControlContext,
        _resources: &mut dyn Resources,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Bitmap,
        _context: &mut ControlContext,
        resources: &mut dyn Resources,
        _size: Size,
    ) {
        self.rect = if let Ok(texture_size) = resources.get_texture_size(data.texture_id.get()) {
            Rect::new(0.0f32, 0.0f32, texture_size.0 as f32, texture_size.1 as f32)
        } else {
            Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32)
        }
    }

    fn set_rect(&mut self, _data: &mut Bitmap, _context: &mut ControlContext, rect: Rect) {
        //self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Bitmap, _context: &ControlContext, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &Bitmap,
        _context: &ControlContext,
        _resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
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

        vec
    }
}
