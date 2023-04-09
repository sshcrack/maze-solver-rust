use minifb::Window;

use crate::{tools::{consts::get_size, window::draw_maze, math::vec2_to_numb}, point::point_state::PointState};

use super::hunt_and_kill;

pub fn generate(window: &mut Window) -> anyhow::Result<()> {
    let size = get_size()?;

    let mut maze = vec![PointState::WALL; size * size];
    hunt_and_kill(&mut maze, window)?;

    /*maze[vec2_to_numb(30, 10, size)] = PointState::PASSAGE;
    maze[vec2_to_numb(31, 11, size)] = PointState::PASSAGE;
    maze[vec2_to_numb(32, 11, size)] = PointState::PASSAGE;

    loop {
        draw_maze(window, &maze)?;
    }*/
    Ok(())
}