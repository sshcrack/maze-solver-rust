use std::{
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};

use anyhow::Result;
use egui::{Color32, Context};

use crate::{
    generators::generate::generate,
    point::{direction::DIRECTION_VEC, point::Point, point_state::VisualIndicator},
    solve::solve::{solve, SolveAlgorithm, SolveOptions},
    tools::{
        consts::{get_size, setup_constants, MazeOptions},
        math::{set_point, set_point_mult},
        matrix::get_pos_between,
        window::update_maze_debug,
    },
};

pub type PixelSize = Arc<RwLock<usize>>;
pub type PixelVector = Arc<RwLock<Vec<Color32>>>;

#[derive(Clone, Debug)]
pub struct Window {
    size: PixelSize,
    pixels: PixelVector,
    ctx: Context,
}

impl Window {
    pub fn new(ctx: &Context, size: &PixelSize, pixels: &PixelVector) -> Self {
        Self {
            ctx: ctx.clone(),
            size: size.clone(),
            pixels: pixels.clone()
        }
    }

    pub fn get_size(&self) -> usize {
        self.size.read().unwrap().clone()
    }

    pub fn set_pixels(&self, pixels: Vec<Color32>) {
        *self.pixels.write().unwrap() = pixels;
        self.ctx.request_repaint();
    }
}

pub struct MazeThread {
    options: MazeOptions,
    thread: JoinHandle<Result<()>>,
}

impl MazeThread {
    pub fn terminate(&self) {
        self.thread.
    }

    pub fn new_default(window: &Window) -> Self {
        return MazeThread::new(
            window,
            &MazeOptions {
                speed: 1.0,
                size: 50,
                seed: rand::random(),
                show_animation: true,
            },
            SolveAlgorithm::AStar
        );
    }

    pub fn new(window: &Window, options: &MazeOptions, algorithm: SolveAlgorithm) -> Self {
        let options = options.clone();

        let temp = options.clone();
        let temp1 = window.clone();
        Self {
            options,
            thread: thread::spawn(move || MazeThread::main_run(temp1, temp, algorithm)),
        }
    }

    pub fn main_run(window: Window, options: MazeOptions, algorithm: SolveAlgorithm) -> Result<()> {
        setup_constants(options);
        let end_coords = get_size()? - 2;

        let mut maze = generate(&window)?;

        let start = Point { x: 1, y: 1 };
        let end = Point {
            x: end_coords,
            y: end_coords,
        };

        let options = SolveOptions {
            algorithm,
            start,
            end,
        };

        let size = get_size()?;
        let path = solve(&mut maze, &window, &options)?;
        let mut visual_overwrites = vec![None as Option<VisualIndicator>; size * size];

        for i in 0..path.len() {
            let next_index = i + 1;
            if next_index == path.len() {
                continue;
            }

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
            set_point_mult(
                &mut visual_overwrites,
                &vec![p, between, n],
                Some(VisualIndicator::SolvePath),
            );
        }

        set_point(&mut visual_overwrites, &start, Some(VisualIndicator::Start));
        set_point(&mut visual_overwrites, &end, Some(VisualIndicator::End));

        update_maze_debug(&window, &maze, &visual_overwrites, true)?;
        Ok(())
    }
}
