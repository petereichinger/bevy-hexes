use std::fmt::Display;

use super::cube::Cube;
use super::offset::Offset;
use bevy::prelude::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Axial {
    pub q: i32,
    pub r: i32,
}

impl Axial {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
}

impl From<Offset> for Axial {
    fn from(offset: Offset) -> Self {
        Self {
            q: offset.col - (offset.row - (offset.row & 1)) / 2,
            r: offset.row,
        }
    }
}

impl From<Axial> for Offset {
    fn from(value: Axial) -> Self {
        Self {
            col: value.q + (value.r - (value.r & 1)) / 2,
            row: value.r,
        }
    }
}

impl From<Cube> for Axial {
    fn from(cube: Cube) -> Self {
        Axial {
            q: cube.q,
            r: cube.r,
        }
    }
}

impl From<Axial> for Vec3 {
    fn from(axial: Axial) -> Self {
        const SIZE: f32 = 0.5;
        let sqrt_3 = f32::sqrt(3.);
        let (q, r) = (axial.q as f32, axial.r as f32);
        let x = SIZE * (sqrt_3 * q + 0.5 * sqrt_3 * r);
        let y = SIZE * (3. / 2. * r);

        Self { x, y: 0.0, z: y }
    }
}

impl Display for Axial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A[{}, {}]", self.q, self.r)
    }
}
