pub use drawing::color::ColorFormat;

use crate::Result;

pub trait Resources {
    fn get_font_dimensions(
        &mut self,
        font_name: &'static str,
        size: u8,
        text: &str,
    ) -> Result<(u16, u16)>;

    fn get_font_dimensions_each_char(
        &mut self,
        font_name: &'static str,
        size: u8,
        text: &str,
    ) -> Result<(Vec<i16>, u16)>;

    fn create_texture(
        &mut self,
        memory: &[u8],
        width: u16,
        height: u16,
        format: ColorFormat,
        updatable: bool,
    ) -> Result<i32>;

    fn update_texture(
        &mut self,
        texture_id: i32,
        memory: &[u8],
        offset_x: u16,
        offset_y: u16,
        width: u16,
        height: u16,
    ) -> Result<()>;

    fn get_texture_size(&mut self, texture_id: i32) -> Result<(u16, u16)>;
}
