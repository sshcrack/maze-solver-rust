use lazy_static::lazy_static;

use crate::tools::direction_data::DirectionData;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::UP,
            Direction::LEFT,
            Direction::RIGHT,
            Direction::DOWN
        ]
    }
}

lazy_static! {
    pub static ref DIRECTION_VEC: Vec<DirectionData> = vec![
        DirectionData::from_tuple(0, -2, Direction::UP),
        DirectionData::from_tuple(-2, 0, Direction::LEFT),
        DirectionData::from_tuple(2, 0, Direction::RIGHT),
        DirectionData::from_tuple(0, 2, Direction::DOWN),
    ];
}
