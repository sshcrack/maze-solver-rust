use std::time::Instant;

use druid::{Data, PaintCtx, Vec2};

use crate::{tool::size::get_size};

use super::point::{face::Face, maze_point::MazePoint};

#[derive(Clone, Data)]
pub struct Maze {
    #[data(ignore)]
    points: Vec<MazePoint>,
    dimension: f64
}

impl Maze {
    pub fn create(dim: usize) -> Self {
        let mut points: Vec<MazePoint> = Vec::new();

        for x in 0..dim*dim {
            points.push(MazePoint::default((x / dim) as f64, (x % dim) as f64));
        }

        return Self {
            points,
            dimension: dim as f64
        };
    }

    pub fn draw(&self, ctx: &mut PaintCtx) {
        let x = Instant::now();

        let size = get_size(ctx);
        let scale = size / self.get_dimension();


        let centered = scale / 2.0;

        for point in &self.points {
            point.draw(ctx, scale, centered, self);
        }

        let duration = x.elapsed();
        println!("Maze render is: {:?}\n", duration);
    }

    pub fn mark_dirty(&self) {
        for ele in self.points.iter() {
            ele.mark_dirty();
        }
    }

    pub fn get_all(&self) -> &Vec<MazePoint> {
        return &self.points;
    }

    pub fn get_all_mut(&mut self) -> &mut Vec<MazePoint> {
        return self.points.as_mut();
    }

    pub fn in_bounds(&self, point: &Vec2) -> bool {
        let x = point.x;
        let y = point.y;

        let x_l = self.dimension;
        let y_l = self.dimension;

        return !(x < 0.0 || y < 0.0 || x >= x_l || y >= y_l);
    }

    pub fn get_dimension(&self) -> f64 {
        return self.dimension;
    }

    pub fn get(&self, pos: Vec2) -> Option<&MazePoint> {
        let x = pos.x;
        let y = pos.y;

        let p = self.points.get(x as usize * (self.dimension as usize) + y as usize);

        return p;
    }

    pub fn get_mut(&mut self, pos: Vec2) -> Option<&mut MazePoint> {
        let x = pos.x;
        let y = pos.y;

        let p = self.points.get_mut(x as usize * (self.dimension as usize) + y as usize);
        return p;
    }

    pub fn get_neighbour_coords(&self, point: &MazePoint, dir: &Face) -> Option<Vec2> {
        let pos = point.get_pos();
        let dir_vec = dir.to_vec();

        let new_pos = pos + dir_vec;
        if self.in_bounds(&new_pos) {
            return None;
        }

        return Some(new_pos);
    }

    pub fn get_neighbour(&self, point: &MazePoint, dir: &Face) -> Option<&MazePoint> {
        let new_pos = self.get_neighbour_coords(point, dir);
        if new_pos.is_none() {
            return None;
        }

        let new_pos = new_pos.unwrap();
        return self.get(new_pos);
    }

    pub fn get_neighbour_mut(&mut self, point: &MazePoint, dir: &Face) -> Option<&mut MazePoint> {
        let new_pos = self.get_neighbour_coords(point, dir);
        if new_pos.is_none() {
            return None;
        }

        let new_pos = new_pos.unwrap();
        return self.get_mut(new_pos);
    }
}
