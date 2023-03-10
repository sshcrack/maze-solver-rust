use std::sync::{mpsc::Sender, Mutex, Arc};

use druid::{PaintCtx};

use super::point::maze_point::MazePoint;


pub struct Maze {
    points: Vec<Vec<MazePoint>>,
    dimension: usize,
    tx: Arc<Mutex<Sender<()>>>
}

impl Maze {
    pub fn create(dim: usize, tx: Arc<Mutex<Sender<()>>>) -> Self {
        let mut points: Vec<Vec<MazePoint>> = Vec::new();

        for x in 0..dim {
            let mut curr_layer: Vec<MazePoint> = Vec::new();
            for y in 0..dim {
                curr_layer.push(MazePoint::default(x, y));
            }

            points.push(curr_layer);
        }
        return Self {
            points,
            dimension: dim,
            tx
        }
    }

    pub fn draw(&self, ctx: &mut PaintCtx) {
        for point_layer in &self.points {
            for point in point_layer {
                point.draw(ctx);
            }
        }
    }

    pub fn get_all(&self) -> Vec<&MazePoint> {
        let mut all = Vec::new();

        for point_layer in &self.points {
            for point in point_layer {
                all.push(point);
            }
        }

        return all;
    }

    pub fn get_all_mut(&mut self) -> Vec<&mut MazePoint> {
        let mut all = Vec::new();

        for point_layer in &mut self.points {
            for point in point_layer {
                all.push(point);
            }
        }

        return all;
    }
}