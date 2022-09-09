use std::fmt::Display;

use super::{axial::Axial, offset::Offset};

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoordinateError {
    #[error("Requirement q + r + s == 0 not fulfilled")]
    SumNotZero,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cube {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl Cube {
    pub fn origin() -> Self {
        Self { q: 0, r: 0, s: 0 }
    }
    pub fn new(q: i32, r: i32, s: i32) -> Result<Self, CoordinateError> {
        let sum = q + r + s;

        match sum {
            0 => Ok(Cube { q, r, s }),
            _ => Err(CoordinateError::SumNotZero),
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
    fn abs(self) -> Self {
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

#[cfg(test)]
mod tests {
    use super::Cube;

    #[test]
    fn creation_of_trivial_coordinate_works() {
        let coord = Cube::new(0, 0, 0);

        assert_eq!(coord, Ok(Cube { q: 0, r: 0, s: 0 }));
    }

    #[test]
    fn creating_invalid_coordinate_fails() {
        let coord = Cube::new(1, 0, 0);

        assert_eq!(coord, Err(CoordinateError::SumNotZero));
    }

    #[test]
    fn subtraction_works() {
        let first = Cube::new(1, 1, -2).unwrap();
        let second = Cube::new(2, 2, -4).unwrap();

        let result = first - second;

        assert_eq!(result, Cube { q: -1, r: -1, s: 2 })
    }

    #[test]
    fn max_component_works() {
        let cube = Cube::new(2, 1, -3).unwrap();

        assert_eq!(cube.max_component(), 2);
    }

    #[test]
    fn distance_same_coord_works() {
        let cube = Cube::new(2, 1, -3).unwrap();

        assert_eq!(cube.distance_to(cube), 0);
    }

    #[test]
    fn distance_neighbour_coord_works() {
        let neighbour = Cube::new(1, 0, -1).unwrap();

        assert_eq!(Cube::origin().distance_to(neighbour), 1);
    }

    #[test]
    fn distance_other_coord_works() {
        let other = Cube::new(1, 2, -3).unwrap();

        assert_eq!(Cube::origin().distance_to(other), 3);
    }
}
