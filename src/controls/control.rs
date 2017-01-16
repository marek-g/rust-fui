use common::size::Size;
use backend::renderer::Renderer;
use render::primitive::Primitive;

pub trait Control {
    fn get_preferred_size(&mut self, size: Size, renderer: &mut Renderer) -> Size;
    fn set_size(&mut self, size: Size, renderer: &mut Renderer) -> Size;

    fn to_primitives(&self) -> Vec<Primitive>;
}