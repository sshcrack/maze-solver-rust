use std::sync::{Arc, RwLock};

use anyhow::Result;
use lazy_static::lazy_static;
use rand::{rngs::StdRng, SeedableRng};

use crate::{point::point_state::PointState};

use super::options::MazeData;

#[derive(Debug, Clone)]
pub struct MazeOptions {
    pub size: usize,
    pub seed: u64,
    seeder: StdRng,
}

impl Default for MazeOptions {
    fn default() -> Self {
        let rand = rand::random();
        Self {
            size: 50,
            seed: rand,
            seeder: StdRng::seed_from_u64(rand)
        }
    }
}

impl MazeOptions {
    pub fn new(size: usize, seed: u64) -> Self {
        Self {
            size,
            seed,
            seeder: StdRng::seed_from_u64(seed)
        }
    }
}

pub type MazeOptionsArc = Arc<RwLock<MazeOptions>>;
pub type FrameCount = Arc<RwLock<u128>>;
pub type Maze = Vec<PointState>;


lazy_static! {
    pub static ref FRAME_COUNT: FrameCount = FrameCount::default();
}


pub fn check_size(data: &MazeData) {
    let mut s = data.get_opt();
    if s.size % 2 == 0 {
        s.size += 1;
    }

    data.write_opt(&s);
}

pub fn get_size(data: &MazeData) -> Result<usize> {
    let opt = data.get_opt();
    return Ok(opt.size);
}

pub fn get_seeder(data: &MazeData) -> StdRng {
    let opt = data.get_opt();
    opt.seeder
}