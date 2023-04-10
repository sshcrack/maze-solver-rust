use std::collections::BinaryHeap;

use anyhow::{anyhow, Result};
use minifb::Window;

use crate::{
    point::{
        direction::Direction,
        point::Point,
        point_state::{PointState, VisualIndicator},
    },
    tools::{
        consts::{Maze, rand_range, get_size},
        math::{get_maze_iter, vec2_to_numb, point_to_numb, get_point, set_point}, window::{update_maze_debug, update_maze}, matrix::{go_to_dir, get_surrounding_walls, get_available_dirs_state}
    },
};

pub fn hunt_and_kill(maze: &mut Maze, window: &mut Window) -> anyhow::Result<()> {
    let size = get_size()?;
    let cell_size = (size - 1) / 2;

    // making sure that passage are always on odd points
    let x = rand_range(0..cell_size) * 2 + 1;
    let y = rand_range(0..cell_size) * 2 + 1;

    let mut pending = BinaryHeap::new();
    pending.push(Point { x, y });

    while !pending.is_empty() {
        let mut p = pending.pop().unwrap();
        let mut dirs = get_surrounding_walls(maze, &p)?;
        let mut adjacent_passages = get_available_dirs_state(maze, &p, PointState::Passage)?;

        if dirs.is_empty() {
            hunt_phase(window, maze, &mut p, &mut dirs, &mut adjacent_passages)?;

            if dirs.is_empty() {
                break;
            }
        }

        let rand_dir = dirs[rand_range(0..dirs.len())];
        let neighbor = go_to_dir(&p, &rand_dir)?;
        if neighbor.is_none() {
            println!("Neighbor null");
            continue;
        }

        let neighbor = neighbor.unwrap();

        remove_wall(maze, &p, &neighbor)?;
        pending.push(neighbor);

        //update_maze(window, maze, false)?;
    }

    for _ in 0..25 {
        update_maze(window, maze, true)?;
    }

    return Ok(());
}

fn hunt_phase(
    window: &mut Window,
    maze: &mut Maze,
    p: &mut Point,
    dirs: &mut Vec<Direction>,
    adjacent_passages: &mut Vec<Direction>,
) -> Result<()> {
    let size = get_size()?;

    let mut should_break = false;
    let mut visual_overwrites = vec![None; size * size];
    for x in get_maze_iter(&size) {
        for y in get_maze_iter(&size) {
            *p = Point { x, y };
            if get_point(maze, &p) == PointState::Wall {
                *adjacent_passages = get_available_dirs_state(maze, p, PointState::Passage)?;
                visual_overwrites[point_to_numb(&p, size)] = Some(VisualIndicator::Searching);

                //update_maze_debug(window, maze, &visual_overwrites, true)?;
                if !adjacent_passages.is_empty() {
                    let passage_dir = adjacent_passages
                        .get(rand_range(0..adjacent_passages.len()))
                        .unwrap();
                    let passage = go_to_dir(&p, passage_dir)?.unwrap();

                    *dirs = get_surrounding_walls(maze, &p)?;
                    if dirs.is_empty() {
                        visual_overwrites[point_to_numb(&p, size)] = Some(VisualIndicator::SolvePath);

                        set_point(maze, &p, PointState::Passage);
                        remove_wall(maze, &p, &passage)?;
                        *adjacent_passages = vec![];
                        continue;
                    }

                    remove_wall(maze, &p, &passage)?;
                    should_break = true;
                    break;
                }
            }
        }

        if should_break {
            break;
        }
    }

    visual_overwrites[point_to_numb(&p, size)] = Some(VisualIndicator::Match);

    for _ in 0..5 {
        update_maze_debug(window, maze, &visual_overwrites, true)?;
    }

    Ok(())
    //println!("Should kill dirs {:?}", dirs);
}

pub fn remove_wall(maze: &mut Maze, from: &Point, to: &Point) -> Result<()> {
    let size = get_size()?;

    if from.x != to.x && from.y != to.y {
        return Err(anyhow!(
            "Points have to be either the same on x or y axis to draw a wall"
        ));
    }

    let Point {
        x: from_x,
        y: from_y,
    } = from;
    let Point { x: to_x, y: to_y } = to;

    let diff_x = (*to_x as i32) - (*from_x as i32);
    let diff_y = (*to_y as i32) - (*from_y as i32);

    let step_x = if diff_x != 0 {
        diff_x / diff_x.abs()
    } else {
        0
    };
    let step_y = if diff_y != 0 {
        diff_y / diff_y.abs()
    } else {
        0
    };

    for curr_x in 0..diff_x.abs() + 1 {
        let correct_x = *from_x as i32 + curr_x * step_x;

        for curr_y in 0..diff_y.abs() + 1 {
            let correct_y = *from_y as i32 + curr_y * step_y;
            if correct_x < 0
                || correct_y < 0
                || correct_x >= size as i32
                || correct_y >= size as i32
            {
                continue;
            }

            let correct_x = correct_x as usize;
            let correct_y = correct_y as usize;

            let ind = vec2_to_numb(correct_x, correct_y, size);
            maze[ind] = PointState::Passage;
        }
    }

    Ok(())
}
