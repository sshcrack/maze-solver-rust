use anyhow::Result;

use crate::{tools::consts::Maze, point::point::Point, manager::Window};

use super::a_star::a_star;

pub fn solve(maze: &mut Maze, window: &Window, options: &SolveOptions) -> Result<Vec<Point>> {
    let SolveOptions { algorithm, .. } = options;
    let res = match algorithm {
        SolveAlgorithm::AStar => a_star(maze, window, options)
    };

    res
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