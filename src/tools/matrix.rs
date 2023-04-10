use anyhow::Result;

use crate::point::{point::Point, point_state::PointState, direction::{Direction, DIRECTION_VEC}};

use super::{consts::{Maze, get_size}, math::get_point, direction_data::DirectionData};


pub fn go_to_dir(point: &Point, dir: &Direction) -> Result<Option<Point>> {
    let mut pos = None;
    let size = get_size()? as i32;
    let Point { x, y } = *point;

    for info in DIRECTION_VEC.iter() {
        if info.dir == *dir {
            let x = x as i32 + info.x;
            let y = y as i32 + info.y;
            if x < 0 || y < 0 || x >= size || y >= size {
                continue;
            }

            let x = x as usize;
            let y = y as usize;
            pos = Some(Point { x, y });
            break;
        }
    }

    return Ok(pos);
}

pub fn get_surrounding_walls(maze: &Maze, point: &Point) -> Result<Vec<Direction>> {
    get_available_dirs_state(maze, point, PointState::Wall)
}

pub fn get_available_dirs_state(
    maze: &Maze,
    point: &Point,
    desired_state: PointState,
) -> anyhow::Result<Vec<Direction>> {
    let mut available = Vec::new();

    for dir in Direction::all() {
        let p = go_to_dir(point, &dir)?;
        if p.is_none() {
            continue;
        }

        let p = p.unwrap();
        let state = get_point(maze, &p);

        if state == desired_state {
            available.push(dir);
        }
    }

    Ok(available)
}

pub fn has_passage_between(maze: &Maze, src: &Point, dir: &Direction) -> Result<Option<bool>> {
    let dest = go_to_dir(src, dir)?;
    if dest.is_none() {
        return Ok(None);
    }

    let DirectionData { x: dir_x, y: dir_y, .. } = dir.to_data().normalize();
    let between = src.add(-dir_x, -dir_y);

    if between.is_none() {
        return Ok(None);
    }

    let between = between.unwrap();
    let state = get_point(maze, &between);

    return Ok(Some(state == PointState::Passage))
}
