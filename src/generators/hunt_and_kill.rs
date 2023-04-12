use std::collections::BinaryHeap;

use anyhow::{anyhow, Result};
use rand::{Rng, rngs::StdRng};
use crate::{
    point::{direction::Direction,point::Point,point_state::{PointState, VisualIndicator}},
    tools::{consts::{Maze, get_size, get_seeder},math::{get_maze_iter, vec2_to_numb, point_to_numb, get_point, set_point}, window::{update_maze_debug, update_maze}, matrix::{go_to_dir, get_surrounding_walls, get_available_dirs_state}, options::MazeData}
};

use super::tools::count_to_percentage;

pub fn hunt_and_kill(maze: &mut Maze, data: &MazeData) -> anyhow::Result<()> {
    let size = get_size(data)?;
    let cell_size = (size - 1) / 2;
    let mut last_percentage = 0.0;
    let mut seeder = get_seeder(data);

    // making sure that passage are always on odd points
    let x = seeder.gen_range(0..cell_size) * 2 + 1;
    let y = seeder.gen_range(0..cell_size) * 2 + 1;

    let mut count = 0 as u64;
    let mut pending = BinaryHeap::new();

    pending.push(Point { x, y });
    while !pending.is_empty() {
        let mut p = pending.pop().unwrap();
        let mut dirs = get_surrounding_walls(&size, maze, &p)?;
        let mut adjacent_passages = get_available_dirs_state(&size, maze, &p, PointState::Passage)?;

        if dirs.is_empty() {
            hunt_phase(
                data,
                &mut seeder,
                maze,
                &mut p,
                &mut dirs,
                &mut adjacent_passages
            )?;

            if dirs.is_empty() { break; }
        }

        let rand_dir = dirs[seeder.gen_range(0..dirs.len())];
        let neighbor = go_to_dir(&size, &p, &rand_dir)?;
        count += 1;
        count_to_percentage(data, size, count, &mut last_percentage).and_then(|e| {
            last_percentage = e;
            data.set_gen_proc(e);
            data.request_repaint();
            println!("Generation: {}%", (e * 100.0 * 100.0).round() / 100.0);
            Some(())
        });

        if neighbor.is_none() { continue; }

        let neighbor = neighbor.unwrap();

        remove_wall(size, maze, &p, &neighbor)?;
        pending.push(neighbor);

        update_maze(data, maze, false)?;
    }

    data.set_gen_proc(1.0);
    data.request_repaint();
    for _ in 0..25 {
        update_maze(data, maze, true)?;
    }

    return Ok(());
}

fn hunt_phase(
    data: &MazeData,
    seeder: &mut StdRng,
    maze: &mut Maze,
    p: &mut Point,
    dirs: &mut Vec<Direction>,
    adjacent_passages: &mut Vec<Direction>
) -> Result<()> {
    let size = get_size(data)?;

    let mut should_break = false;
    let show_anim = data.show_anim();

    let desired_size = if show_anim { size * size } else { 0 };
    let mut visual_overwrites = vec![None; desired_size];
    for x in get_maze_iter(&size) {
        for y in get_maze_iter(&size) {
            *p = Point { x, y };
            if get_point(maze, &p) == PointState::Wall {
                *adjacent_passages = get_available_dirs_state(&size, maze, p, PointState::Passage)?;
                if show_anim  {
                    visual_overwrites[point_to_numb(&p, size)] = Some(VisualIndicator::Searching);
                }

                if !adjacent_passages.is_empty() {
                    let passage_dir = adjacent_passages
                        .get(seeder.gen_range(0..adjacent_passages.len()))
                        .unwrap();
                    let passage = go_to_dir(&size, &p, passage_dir)?.unwrap();

                    *dirs = get_surrounding_walls(&size, maze, &p)?;
                    if dirs.is_empty() {
                        if show_anim {
                            visual_overwrites[point_to_numb(&p, size)] = Some(VisualIndicator::SolvePath);
                        }

                        set_point(maze, &p, PointState::Passage);
                        remove_wall(size, maze, &p, &passage)?;
                        *adjacent_passages = vec![];
                        continue;
                    }

                    remove_wall(size, maze, &p, &passage)?;
                    should_break = true;
                    break;
                }
            }
        }

        if should_break {
            break;
        }
    }

    if show_anim {
        visual_overwrites[point_to_numb(&p, size)] = Some(VisualIndicator::Match);
    }
    for _ in 0..5 {
        update_maze_debug(data, maze, &visual_overwrites, false)?;
    }

    Ok(())
    //println!("Should kill dirs {:?}", dirs);
}

pub fn remove_wall(size: usize, maze: &mut Maze, from: &Point, to: &Point) -> Result<()> {
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
