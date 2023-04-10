use anyhow::Result;
use minifb::Window;

use crate::{tools::consts::Maze, point::point::Point};

use super::a_star::a_star;

pub fn solve(maze: &mut Maze, window: &mut Window, options: &SolveOptions) -> Result<()> {
    let SolveOptions { algorithm, .. } = options;
    let res = match algorithm {
        SolveAlgorithm::AStar => a_star(maze, window, options)
    };

    res?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolveAlgorithm {
    AStar
}

#[derive(Debug, Clone, Copy)]
pub struct SolveOptions {
    pub start: Point,
    pub end: Point,
    pub algorithm: SolveAlgorithm
}