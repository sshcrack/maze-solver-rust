use crate::{tools::consts::{get_size, Maze}, point::point_state::PointState, manager::Window};

use super::hunt_and_kill;

pub fn generate(window: &Window) -> anyhow::Result<Maze> {
    let size = get_size()?;

    let mut maze = vec![PointState::Wall; size * size];
    hunt_and_kill(&mut maze, window)?;

    Ok(maze)
}