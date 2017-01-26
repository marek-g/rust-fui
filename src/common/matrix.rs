use std::ops::Add;
use std::ops::Mul;
use std::marker::Copy;

pub type Vector2<T> = [T; 2];
pub type Vector3<T> = [T; 3];
pub type Point2<T> = [T; 2];

#[derive(Debug, Clone, Copy)]
pub struct Matrix2x3<T> where T: Copy {
    pub row: [[T; 3]; 2]
}

impl<T> Matrix2x3<T> where T: Copy {
    pub fn new(r0c0: T, r0c1: T, r0c2: T,
        r1c0: T, r1c1: T, r1c2: T) -> Matrix2x3<T> {
        Matrix2x3 { row: [[ r0c0, r0c1, r0c2 ], [ r1c0, r1c1, r1c2 ]] }
    }
}

impl<T> Mul<Matrix2x3<T>> for Matrix2x3<T> where T: Mul<Output=T> + Add<Output=T> + Copy {
    type Output = Matrix2x3<T>;
    fn mul(self, other: Matrix2x3<T>) -> Matrix2x3<T> {
        Matrix2x3 { row: [
            [
                self.row[0][0]*other.row[0][0] + self.row[0][1]*other.row[1][0],
                self.row[0][0]*other.row[0][1] + self.row[0][1]*other.row[1][1],
                self.row[0][0]*other.row[0][2] + self.row[0][1]*other.row[1][2],
            ],
            [
                self.row[1][0]*other.row[0][0] + self.row[1][1]*other.row[1][0],
                self.row[1][0]*other.row[0][1] + self.row[1][1]*other.row[1][1],
                self.row[1][0]*other.row[0][2] + self.row[1][1]*other.row[1][2]
            ]
        ] }
    }
}

impl<T> Mul<Vector2<T>> for Matrix2x3<T> where T: Mul<Output=T> + Add<Output=T> + Copy {
    type Output = Vector2<T>;
    fn mul(self, other: Vector2<T>) -> Vector2<T> {
        [
            self.row[0][0]*other[0] + self.row[0][1]*other[1] + self.row[0][2],
            self.row[1][0]*other[0] + self.row[1][1]*other[1] + self.row[1][2],
        ]
    }
}
