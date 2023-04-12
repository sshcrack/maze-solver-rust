use crate::{tools::{consts::{get_size, Maze}, options::MazeData}, point::point_state::PointState};

use super::hunt_and_kill;

pub fn generate(data: &MazeData) -> anyhow::Result<(i32, Maze)> {
    let size = get_size(data)?;

    let mut maze = vec![PointState::Wall; size * size];
    let count = hunt_and_kill(&mut maze, data)?;

    Ok((count, maze))
}