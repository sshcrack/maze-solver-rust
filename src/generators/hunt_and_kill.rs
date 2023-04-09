use std::{
    collections::BinaryHeap,
    sync::{Arc, RwLock},
};

use anyhow::{anyhow, Result};
use minifb::Window;
use rand::Rng;

use crate::{
    point::{
        direction::{Direction, DIRECTION_VEC},
        point::Point,
        point_state::PointState,
    },
    tools::{
        consts::{get_size, Maze, rand_range},
        math::{get_maze_iter, get_point_vec, set_point_vec2, vec2_to_numb, point_to_numb},
        window::{draw_maze, draw_maze_overwrites},
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
        let mut dirs = get_available_dirs(maze, &p)?;
        let mut adjacent_passages = get_available_dirs_state(maze, &p, PointState::Passage)?;

        if dirs.is_empty() {
            hunt_phase(window, maze, &mut p, &mut dirs, &mut adjacent_passages)?;

            if dirs.is_empty() {
                break;
            }
        }

        let rand_dir = dirs[rand_range(0..dirs.len())];
        let neighbor = go_to_dir(maze, &p, &rand_dir)?;
        if neighbor.is_none() {
            println!("Neighbor null");
            continue;
        }

        let neighbor = neighbor.unwrap();

        remove_wall(maze, &p, &neighbor)?;
        pending.push(neighbor);

        /*println!(
            "from {:?} to {:?} {:?} dir {:?}",
            p,
            neighbor,
            get_point_vec(maze, &neighbor, size),
            rand_dir
        );*/
        for i in 0..2 {
            draw_maze(window, maze)?;
        }
    }

    loop {
        draw_maze(window, maze)?;
    }
}

fn hunt_phase(
    window: &mut Window,
    maze: &mut Maze,
    p: &mut Point,
    dirs: &mut Vec<Direction>,
    adjacent_passages: &mut Vec<Direction>,
) -> Result<()> {
    let mut rng = rand::thread_rng();
    let size = get_size()?;

    let mut should_break = false;
    let mut visual_overwrites = vec![None; size * size];
    for x in get_maze_iter(&size) {
        for y in get_maze_iter(&size) {
            *p = Point { x, y };
            if get_point_vec(maze, &p, size) == PointState::Wall {
                *adjacent_passages = get_available_dirs_state(maze, p, PointState::Passage)?;
                visual_overwrites[point_to_numb(&p, size)] = Some(PointState::Highlight);
                if rng.gen_bool(0.3) {
                    draw_maze_overwrites(window, maze, &visual_overwrites)?;
                }

                if !adjacent_passages.is_empty() {
                    println!("Adjacent are {:?}", adjacent_passages);
                    let passage_dir = adjacent_passages
                        .get(rand_range(0..adjacent_passages.len()))
                        .unwrap();
                    let passage = go_to_dir(maze, &p, passage_dir)?.unwrap();

                    *dirs = get_available_dirs(maze, &p)?;
                    println!("Dirs are {:?} adjacent: {:?}", dirs, adjacent_passages);
                    if dirs.is_empty() {
                        println!("Dirs are empty.");
                        visual_overwrites[point_to_numb(&p, size)] = Some(PointState::SolvePath);

                        set_point_vec2(maze, &p, size, PointState::Passage);
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
            println!("Breaking");
            break;
        }
    }

    visual_overwrites[point_to_numb(&p, size)] = Some(PointState::HighlightSecondary);
    for _ in 0..10 {
        draw_maze_overwrites(window, maze, &visual_overwrites)?;
    }

    Ok(())
    //println!("Should kill dirs {:?}", dirs);
}

pub fn go_to_dir(maze: &Maze, point: &Point, dir: &Direction) -> Result<Option<Point>> {
    let mut pos = None;
    let size = get_size()? as i32;
    let Point { x, y } = *point;

    for info in &*DIRECTION_VEC {
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

pub fn get_available_dirs(maze: &Maze, point: &Point) -> anyhow::Result<Vec<Direction>> {
    get_available_dirs_state(maze, point, PointState::Wall)
}

pub fn get_available_dirs_state(
    maze: &Maze,
    point: &Point,
    desired_state: PointState,
) -> anyhow::Result<Vec<Direction>> {
    let mut available = Vec::new();
    let size = get_size()?;

    for dir in Direction::all() {
        let p = go_to_dir(maze, point, &dir)?;
        if p.is_none() {
            continue;
        }

        let p = p.unwrap();
        let state = get_point_vec(maze, &p, size);

        if state == desired_state {
            available.push(dir);
        }
    }

    Ok(available)
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
