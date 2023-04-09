use crate::point::direction::Direction;

// Used just to store relative data to check if neighbors are alright
#[derive(Debug)]
pub struct DirectionData {
    pub x: i32,
    pub y: i32,
    pub dir: Direction
}

impl DirectionData {
    pub fn from_tuple(x: i32, y: i32, dir: Direction) -> Self {
        Self {
            x, y, dir
        }
    }
}