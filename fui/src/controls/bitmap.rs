use std::cell::{ RefCell, RefMut };
use std::rc::{ Rc, Weak };

use control::*;
use control_object::*;
use common::*;
use drawing_context::DrawingContext;
use drawing::backend::Texture;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use events::*;
use observable::*;
use Property;

pub struct Bitmap {
    texture_id: Property<i32>,
}

impl Bitmap {
    pub fn new(texture_id: i32) -> Self {
        Bitmap {
            texture_id: Property::new(texture_id),
        }
    }

    pub fn control(texture_id: i32) -> Rc<RefCell<Control<Self>>> {
        Control::new(
            BitmapDefaultStyle::new(),
            Self::new(texture_id),
        )
    }
}

impl ControlBehaviour for Control<Bitmap> {
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        Vec::new()
    }

    fn handle_event(&mut self, _event: ControlEvent) { }
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
            rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<Bitmap> for BitmapDefaultStyle {
    fn setup_dirty_watching(&mut self, data: &mut Bitmap, control: &Rc<RefCell<Control<Bitmap>>>) {
        self.event_subscriptions.push(data.texture_id.dirty_watching(control));
    }

    fn get_preferred_size(&self, data: &Bitmap, drawing_context: &mut DrawingContext, _size: Size) -> Size {
        if let Some(texture) = drawing_context.get_resources().textures().get(&data.texture_id.get()) {
            let size = texture.get_size();
            Size::new(size.0 as f32, size.1 as f32)
        } else {
            Size::new(0.0f32, 0.0f32)
        }
    }

    fn set_rect(&mut self, _data: &Bitmap, rect: Rect) {    
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Bitmap, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) { HitTestResult::Current } else { HitTestResult::Nothing }
    }

    fn to_primitives(&self, data: &Bitmap,
        _drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        if self.rect.width > 0.0f32 && self.rect.height > 0.0f32 {
            vec.push(Primitive::Image {
                resource_key: data.texture_id.get(),
                rect: UserPixelRect::new(
                    UserPixelPoint::new(self.rect.x, self.rect.y),
                    UserPixelSize::new(self.rect.width, self.rect.height))
            });
        }

        vec
    }
}
