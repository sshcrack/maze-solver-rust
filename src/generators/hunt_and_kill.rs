use std::{collections::BinaryHeap, cmp::{min, max}};

use anyhow::{Result, anyhow};
use minifb::Window;
use rand::Rng;

use crate::{
    point::{direction::{Direction, DIRECTION_VEC}, point::Point, point_state::PointState},
    tools::{
        consts::{get_size, Maze},
        math::{get_point_vec, vec2_to_numb, set_point_vec2},
        window::draw_maze,
    },
};

pub fn hunt_and_kill(maze: &mut Maze, window: &mut Window) -> anyhow::Result<()> {
    let size = get_size()?;
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0..size);
    let y = rng.gen_range(0..size);

    let mut pending = BinaryHeap::new();
    pending.push(Point { x, y });

    while !pending.is_empty() {
        let mut p = pending.pop().unwrap();
        let mut dirs = get_available_directions(maze, &p)?;

        if dirs.is_empty() {
            while dirs.is_empty() {
                for x in 0..size {
                    for y in 0..size {
                        p = Point { x, y };
                        if get_point_vec(maze, &p, size) == PointState::WALL {
                            dirs = get_available_directions(maze, &p)?;
                        }
                    }
                }
            }
            println!("Should kill dirs {:?}", dirs);
        }

        let rand_dir = dirs[rng.gen_range(0..dirs.len())];
        let neighbor = go_to_dir(maze, &p, &rand_dir)?;
        if neighbor.is_none() {
            println!("Neighbor null");
            continue;
        }

        let neighbor = neighbor.unwrap();

        remove_wall(maze, &p, &neighbor)?;
        pending.push(neighbor);
        set_point_vec2(maze, &p, size, PointState::VISITED);

        println!("from {:?} to {:?} {:?} dir {:?}", p, neighbor, get_point_vec(maze, &neighbor, size), rand_dir);
        for i in 0..25 {
            draw_maze(window, maze)?;
        }
    }

    loop {
        draw_maze(window, maze)?;
    }
}

pub fn go_to_dir(maze: &Maze, point: &Point, dir: &Direction) -> Result<Option<Point>> {
    let mut pos = None;
    let size = get_size()? as i32;
    let Point { x, y } = *point;

    for info in &*DIRECTION_VEC {
        if info.dir == *dir {
            let x = x as i32 + info.x;
            let y = y as i32 + info.y;
            if x < 0 || y < 0 || x >= size || y >= size { continue; }

            let x = x as usize;
            let y = y as usize;
            pos = Some(Point { x, y });
            break;
        }
    }

    return Ok(pos);
}

pub fn get_available_directions(maze: &Maze, point: &Point) -> anyhow::Result<Vec<Direction>> {
    let mut available = Vec::new();
    let size = get_size()?;

    for dir in Direction::all() {
        let p = go_to_dir(maze, point, &dir)?;
        if p.is_none() {
            continue;
        }


        let p = p.unwrap();
        let state = get_point_vec(maze, &p, size);

        if state == PointState::WALL { available.push(dir); }
    }

    Ok(available)
}

pub fn remove_wall(maze: &mut Maze, from: &Point, to: &Point) -> Result<()>{
    let size = get_size()?;

    if from.x != to.x && from.y != to.y {
        return Err(anyhow!("Points have to be either the same on x or y axis to draw a wall"))
    }

    let Point { x: from_x, y: from_y } = from;
    let Point { x: to_x, y: to_y} = to;

    let diff_x = (*to_x as i32) - (*from_x as i32);
    let diff_y = (*to_y as i32) - (*from_y as i32);

    let step_x = if diff_x != 0 { diff_x / diff_x.abs() } else { 0 };
    let step_y = if diff_y != 0 { diff_y / diff_y.abs() } else { 0 };

    println!("diff {} {} step {} {}", diff_x, diff_y, step_x, step_y);
    for curr_x in 0..diff_x.abs() +1 {
        let correct_x = *from_x as i32 + curr_x * step_x;

        for curr_y in 0..diff_y.abs() +1 {
            let correct_y = *from_y as i32 + curr_y * step_y;
            if correct_x < 0 || correct_y < 0 || correct_x >= size as i32 || correct_y >= size as i32 {
                continue;
            }

            let correct_x = correct_x as usize;
            let correct_y = correct_y as usize;

            let ind = vec2_to_numb(correct_x, correct_y, size);
            maze[ind] = PointState::PASSAGE;
        }
    }


    Ok(())
}