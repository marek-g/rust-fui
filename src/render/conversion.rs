use controls::control::Control;
use render::primitive::Primitive;

pub fn convert_control_to_primitives(control: &Control) -> Vec<Primitive> {
    control.to_primitives()
}