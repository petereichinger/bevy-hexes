use std::fmt::Display;

use super::{axial::Axial, offset::Offset};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cube {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl Cube {
    pub fn new(q: i32, r: i32, s: i32) -> Option<Self> {
        let sum = q + r + s;

        match sum {
            0 => Some(Cube { q, r, s }),
            _ => None,
        }
    }
}

impl std::ops::Sub<Cube> for Cube {
    type Output = Cube;

    fn sub(self, rhs: Cube) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}

impl Cube {
    pub fn abs(self) -> Self {
        Self {
            q: self.q.abs(),
            r: self.r.abs(),
            s: self.s.abs(),
        }
    }

    pub fn max_component(self) -> i32 {
        return self.q.max(self.r).max(self.s);
    }

    pub fn distance_to(self, other: Self) -> u32 {
        (self - other).abs().max_component() as u32
    }
}

impl From<Axial> for Cube {
    fn from(axial: Axial) -> Self {
        let (q, r) = (axial.q, axial.r);

        Cube { q, r, s: -q - r }
    }
}

impl From<Cube> for Offset {
    fn from(cube: Cube) -> Self {
        Offset::from(Axial::from(cube))
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C[{}, {}, {}]", self.q, self.r, self.s)
    }
}
