use std::fmt::Display;

use bevy::prelude::IVec2;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Offset {
    pub col: i32,
    pub row: i32,
}

impl Offset {
    pub fn new(col: i32, row: i32) -> Self {
        Self { col, row }
    }
}

impl From<IVec2> for Offset {
    fn from(vec: IVec2) -> Self {
        Self {
            col: vec.x,
            row: vec.y,
        }
    }
}

impl From<Offset> for IVec2 {
    fn from(val: Offset) -> Self {
        Self {
            x: val.col,
            y: val.row,
        }
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "O[{}, {}]", self.col, self.row)
    }
}
