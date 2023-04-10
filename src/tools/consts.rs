use std::{sync::{Arc, RwLock}, ops::Range};

use anyhow::Result;
use lazy_static::lazy_static;
use rand::{rngs::StdRng, SeedableRng, Rng};

use crate::point::point_state::PointState;

#[derive(Debug, Clone, Copy)]
pub struct MazeOptions {
    pub speed: f64,
    pub size: usize,
    pub seed: u64,
    pub show_animation: bool
}

impl Default for MazeOptions {
    fn default() -> Self {
        Self {
            speed: 1.0,
            size: 50,
            seed: rand::random(),
            show_animation: true
        }
    }
}

pub type MazeOptionsArc = Arc<RwLock<MazeOptions>>;
pub type FrameCount = Arc<RwLock<u128>>;
pub type MazeSeeder = Arc<RwLock<StdRng>>;
pub type Maze = Vec<PointState>;


lazy_static! {
    pub static ref MAZE_OPTIONS: MazeOptionsArc = MazeOptionsArc::default();

    pub static ref FRAME_COUNT: FrameCount = FrameCount::default();
    pub static ref MAZE_SEEDER: MazeSeeder = Arc::new(RwLock::new(StdRng::seed_from_u64(0)));
}


pub fn setup_constants(mut opt: MazeOptions) {
    let mut s = MAZE_OPTIONS.write().unwrap();
    if opt.size % 2 == 0 {
        opt.size += 1;
    }

    *s = opt;

    drop(s);


    let mut s = MAZE_SEEDER.write().unwrap();
    *s = StdRng::seed_from_u64(opt.seed);

    drop(s);
}

pub fn get_options() -> Result<MazeOptions> {
    let s = MAZE_OPTIONS.read().unwrap();
    let opt = s.clone();

    drop(s);

    return Ok(opt);
}

pub fn get_size() -> Result<usize> {
    let opt = get_options()?;

    return Ok(opt.size);
}

pub fn rand_range(r: Range<usize>) -> usize {
    let mut rng = MAZE_SEEDER.write().unwrap();
    let out = rng.gen_range(r).clone();
    drop(rng);

    return out;
}