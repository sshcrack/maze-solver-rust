use minifb::Window;

use crate::{tools::consts::get_size, point::point_state::PointState};

use super::hunt_and_kill;

pub fn generate(window: &mut Window) -> anyhow::Result<()> {
    let size = get_size()?;

    let mut maze = vec![PointState::WALL; size];
    maze = hunt_and_kill(maze, window)?;

    Ok(())
}