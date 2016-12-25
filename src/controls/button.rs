use controls::control::Control;
use render::primitive::{Primitive, PrimitiveKind};

pub struct Button {

}

impl Control for Button {

    fn to_primitives(&self) -> Vec<Primitive> {
        let mut vec = Vec::new();
        vec.push(Primitive {
            kind: PrimitiveKind::Rectangle {
                color: [0.5, 1.0, 0.0, 0.0],
                x: 100.0, y: 200.0,
                width: 100.0, height: 200.0
            }
        });
        vec
    }

}