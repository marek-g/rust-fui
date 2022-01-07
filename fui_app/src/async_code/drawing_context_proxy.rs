use fui_core::{DrawingContext, Resources};
use std::sync::Arc;

pub struct DrawingContextProxy {
    drawing_context_arc: Arc<dyn DrawingContext>,
    drawing_area_size: (u16, u16),
    background_texture: i32,
}

impl DrawingContext for DrawingContextProxy {
    fn get_drawing_area_size(&self) -> (u16, u16) {
        todo!()
    }

    fn get_resources(&mut self) -> &mut dyn Resources {
        todo!()
    }

    fn get_background_texture(&self) -> i32 {
        todo!()
    }
}
