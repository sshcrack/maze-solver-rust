use anyhow::Result;

use crate::{tools::{consts::Maze, options::MazeData}, point::point::Point};

use super::a_star::a_star;

pub fn solve(maze: &mut Maze, data: &MazeData, options: &SolveOptions) -> Result<Vec<Point>> {
    let SolveOptions { algorithm, .. } = options;
    let res = match algorithm {
        SolveAlgorithm::AStar => a_star(maze, data, options)
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