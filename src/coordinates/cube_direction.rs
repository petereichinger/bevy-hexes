use strum::EnumIter;

use super::Cube;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    E,
    NE,
    NW,
    W,
    SW,
    SE,
}

impl From<Direction> for Cube {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::E => Cube { q: 1, r: 0, s: -1 },
            Direction::NE => Cube { q: 1, r: -1, s: 0 },
            Direction::NW => Cube { q: 0, r: -1, s: 1 },
            Direction::W => Cube { q: -1, r: 0, s: 1 },
            Direction::SW => Cube { q: -1, r: 1, s: 0 },
            Direction::SE => Cube { q: 0, r: 1, s: -1 },
        }
    }
}

impl std::ops::Add<Direction> for Cube {
    type Output = Cube;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Cube::from(rhs)
    }
}
