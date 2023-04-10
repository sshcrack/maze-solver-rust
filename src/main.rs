use generators::generate::generate;
use point::point::Point;
use solve::solve::{solve, SolveOptions, SolveAlgorithm};
use tools::{consts::{setup_constants, MazeOptions, get_size}, window::{setup_window, update_maze}};
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

    let options = SolveOptions {
        algorithm: SolveAlgorithm::AStar,
        start: Point { x: 1, y: 1 },
        end: Point { x: end_coords, y: end_coords }
    };

    solve(&mut maze, &mut window, &options)?;

    loop {
        update_maze(&mut window, &maze, true)?;
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