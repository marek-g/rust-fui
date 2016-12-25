use render::primitive::Primitive;

pub trait Control {
    fn to_primitives(&self) -> Vec<Primitive>;
}