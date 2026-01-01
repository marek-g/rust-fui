use fui_drawing::{DrawingDisplayListBuilder, DrawingFonts};

pub struct FuiDrawingContext<'a> {
    // currently ViewModel's thread is different than GUI's thread
    // and cannot use DrawingContextGl from ViewModel's thread
    // TODO: maybe put here some proxy object to DrawingContextGl
    // to allow creating textures and fragment programs
    //pub context: &'a mut DrawingContextGl,
    pub fonts: &'a mut DrawingFonts,

    /// Here controls can draw it's content
    pub display: &'a mut DrawingDisplayListBuilder,

    /// Here modal dialogs or overlay controls can draw it's content
    pub overlay: &'a mut DrawingDisplayListBuilder,
}

/*impl FuiDrawingContext<C>
where
    C: drawing_api::Context,
{
    fn get_drawing_area_size(&self) -> (u16, u16);
    fn get_resources(&mut self) -> &mut dyn Resources;
    fn get_background_texture(&self) -> i32;
}
 */
