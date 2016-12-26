use controls::control::Control;
use render::primitive::{Primitive, PrimitiveKind};

pub struct Button {

}

impl Control for Button {

    fn to_primitives(&self) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = 200.0;
        let y = 100.0;
        let width = 200.0;
        let height = 50.0;

        vec.push(Primitive {
            kind: PrimitiveKind::Rectangle {
                color: [0.5, 1.0, 0.0, 0.0],
                x: x, y: y,
                width: width - 1.0, height: height - 1.0
            }
        });

        vec.push(Primitive  {
            kind: PrimitiveKind::Line {
                color: [1.0, 1.0, 1.0, 1.0],
                thickness: 1.0,
                x1: x, y1: y + height - 1.0,
                x2: x, y2: y,
            }
        });
        vec.push(Primitive  {
            kind: PrimitiveKind::Line {
                color: [1.0, 1.0, 1.0, 1.0],
                thickness: 1.0,
                x1: x, y1: y,
                x2: x + width - 1.0, y2: y,
            }
        });
        vec.push(Primitive  {
            kind: PrimitiveKind::Line {
                color: [1.0, 0.0, 0.0, 0.0],
                thickness: 1.0,
                x1: x, y1: y + height - 1.0,
                x2: x + width - 1.0, y2: y + height - 1.0,
            }
        });
        vec.push(Primitive  {
            kind: PrimitiveKind::Line {
                color: [1.0, 0.0, 0.0, 0.0],
                thickness: 1.0,
                x1: x + width - 1.0, y1: y + height - 1.0,
                x2: x + width - 1.0, y2: y,
            }
        });

        vec
    }

}