use common::size::Size;
use application::Application;
use drawing::primitive::Primitive;

pub trait Control {
    fn get_preferred_size(&mut self, size: Size, app: &mut Application) -> Size;
    fn set_size(&mut self, size: Size, app: &mut Application) -> Size;

    fn to_primitives(&self) -> Vec<Primitive>;
}