use crate::point::{point::Point, point_state::PointState};

use super::consts::{Maze, get_size};

pub fn vec2_to_numb(x: usize, y: usize, size: usize) -> usize {
    return y * size + x;
}

pub fn numb_to_vec2(numb: usize, size: usize) -> (usize, usize) {
    return (numb % size, numb / size);
}

pub fn get_point_vec(maze: &Maze, pos: &Point, size: usize) -> PointState {
    let pos = vec2_to_numb(pos.x, pos.y, size);

    maze[pos]
}

pub fn get_point(maze: &Maze, x: usize, y: usize, size: usize) -> PointState {
    let pos = vec2_to_numb(x, y, size);

    maze[pos]
}

pub fn set_point(maze: &mut Maze, x: usize, y: usize, size: usize, state: PointState) {
    maze[vec2_to_numb(x, y, size)] = state;
}

pub fn set_point_vec2(maze: &mut Maze, pos: &Point, size: usize, state: PointState) {
    maze[vec2_to_numb(pos.x, pos.y, size)] = state;
}