use generators::generate::generate;
use point::{point::Point, point_state::VisualIndicator, direction::DIRECTION_VEC};
use solve::solve::{solve, SolveOptions, SolveAlgorithm};
use tools::{consts::{setup_constants, MazeOptions, get_size}, window::{setup_window, update_maze_debug}, matrix::get_pos_between, math::{set_point, set_point_mult}};
use std::io::{stdin, stdout, Read, Write};

mod tools;
mod generators;
mod solve;
mod point;


fn main_run() -> anyhow::Result<()> {
    let scale = 5;
    let speed = 1.0;
    let size = 50;
    let seed: u64 = rand::random();

    setup_constants(MazeOptions {
        scale,
        speed,
        size,
        seed,
        show_animation: true
    });

    let end_coords = get_size()? -2;

    let mut window = setup_window()?;
    let mut maze = generate(&mut window)?;

    let start = Point { x: 1, y: 1 };
    let end = Point { x: end_coords, y: end_coords };

    let options = SolveOptions {
        algorithm: SolveAlgorithm::AStar,
        start,
        end
    };

    let size = get_size()?;
    let path = solve(&mut maze, &mut window, &options)?;
    let mut visual_overwrites = vec![None as Option<VisualIndicator>; size * size];

    for i in 0..path.len() {
        let next_index = i + 1;
        if next_index == path.len() { continue; }

        let p = path[i];
        let n = path[next_index];

        let mut dir = None;
        for el in DIRECTION_VEC.iter() {
            let x = n.x as i32 - p.x as i32;
            let y = n.y as i32 - p.y as i32;

            if x == el.x && y == el.y {
                dir = Some(el.dir);
            }
        }

        if dir.is_none() {
            eprintln!("Could not find direction for {:?} to {:?}6", p, n);
            continue;
        }

        let between = get_pos_between(&p, &dir.unwrap())?.unwrap();
        set_point_mult(&mut visual_overwrites, &vec![p, between, n], Some(VisualIndicator::SolvePath));
    }

    set_point(&mut visual_overwrites, &start, Some(VisualIndicator::Start));
    set_point(&mut visual_overwrites, &end, Some(VisualIndicator::End));

    loop {
        update_maze_debug(&mut window, &maze, &visual_overwrites, true)?;
    }
}


pub fn main() {
    let res = main_run();
    if res.is_err() {
        eprintln!("{}", res.unwrap_err());
    } else {
        println!("Done.");
    }

    pause();
}


fn pause()
{
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}