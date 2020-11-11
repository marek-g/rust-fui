use crate::Resources;

pub trait DrawingContext {
    fn get_drawing_area_size(&self) -> (u16, u16);
    fn get_resources(&mut self) -> &mut dyn Resources;
    fn get_background_texture(&self) -> i32;
}
