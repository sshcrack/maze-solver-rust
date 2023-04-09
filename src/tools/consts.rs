use std::sync::{Arc, RwLock};

use anyhow::{Result, anyhow};
use lazy_static::lazy_static;

use crate::point::point_state::PointState;

pub type AnimationScale = Arc<RwLock<usize>>;
pub type AnimationSpeed = Arc<RwLock<f64>>;
pub type MazeSize = Arc<RwLock<usize>>;
pub type Maze = Vec<PointState>;


lazy_static! {
    pub static ref ANIMATION_SCALE: AnimationScale = AnimationScale::default();
    pub static ref ANIMATION_SPEED: AnimationSpeed = AnimationSpeed::default();
    pub static ref MAZE_SIZE: MazeSize = MazeSize::default();
}


pub fn setup_constants() {
    let mut s = ANIMATION_SCALE.write().unwrap();
    *s = 5;
    drop(s);

    let mut s = ANIMATION_SPEED.write().unwrap();
    *s = 1.0;

    drop(s);

    let mut s = MAZE_SIZE.write().unwrap();
    *s = 50;

    drop(s);
}

pub fn get_scale() -> Result<usize> {
    let scale = ANIMATION_SCALE.read().or(Err(anyhow!("Error reading scale")))?;

    return Ok(scale.clone());
}

pub fn get_speed() -> Result<f64> {
    let e = ANIMATION_SPEED.read().or(Err(anyhow!("Error reading speed")))?;
    return Ok(e.clone());
}

pub fn get_size() -> Result<usize> {
    let s = MAZE_SIZE.read().or(Err(anyhow!("Error reading size")))?;
    let mut size = s.clone();

    drop(s);
    if size % 2 == 0 {
        let mut s = MAZE_SIZE.write().or(Err(anyhow!("Error writing size")))?;
        size += 1;
        *s = size;

        drop(s);
    }

    return Ok(size);
}

pub fn get_window_size() -> Result<usize> {
    return Ok(get_scale()? * get_size()?);
}