use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use fui_core::*;
use fui_drawing::prelude::*;
use typed_builder::TypedBuilder;

#[derive(Clone)]
pub struct BitmapTexture {
    pub texture: Arc<DrawingTexture>,
}

impl PartialEq for BitmapTexture {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.texture, &other.texture)
    }
}

#[derive(TypedBuilder)]
pub struct Bitmap {
    pub texture: Property<Option<BitmapTexture>>,
}

impl Bitmap {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
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

pub struct DefaultBitmapStyle;

impl DefaultBitmapStyle {
    pub fn new(_params: DefaultBitmapStyleParams) -> Self {
        DefaultBitmapStyle {}
    }
}

impl Style<Bitmap> for DefaultBitmapStyle {
    fn setup(&mut self, data: &mut Bitmap, control_context: &mut ControlContext) {
        control_context.dirty_watch_property(&data.texture);
    }

    fn handle_event(
        &mut self,
        _data: &mut Bitmap,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Bitmap,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _size: Size,
    ) -> Size {
        match data.texture.get() {
            Some(texture) => {
                let descriptor = texture.texture.get_descriptor();
                Size::new(descriptor.width as f32, descriptor.height as f32)
            }
            None => Size::new(0.0f32, 0.0f32),
        }
    }

    fn set_rect(
        &mut self,
        _data: &mut Bitmap,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _rect: Rect,
    ) {
    }

    fn hit_test(
        &self,
        _data: &Bitmap,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        data: &Bitmap,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let rect = control_context.get_rect();
        if rect.width > 0.0f32 && rect.height > 0.0f32 {
            match data.texture.get() {
                Some(texture) => {
                    drawing_context.display.draw_texture(
                        texture.texture.as_ref(),
                        (rect.x, rect.y),
                        TextureSampling::Linear,
                        None,
                    );
                }
                None => (),
            }
        }
    }
}
