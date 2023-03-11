use std::sync::{RwLock, Arc};

use druid::{Color, PaintCtx, RenderContext, Vec2, kurbo::Circle};

use crate::{path::maze::Maze, consts::POINT_RADIUS};

use super::face::Face;

#[derive(Clone)]
pub struct MazePoint {
    pub color: Color,
    faces: Vec<Face>,
    x: f64,
    y: f64,
    to_draw_cache: Arc<RwLock<Vec<Face>>>
}

impl MazePoint {
    pub fn default(x: f64, y: f64) -> Self {
        return Self {
            color: Color::RED,
            faces: vec![Face::LEFT],
            x,
            y,
            to_draw_cache: Arc::new(RwLock::new(Vec::new()))
        };
    }

    pub fn update_cache(&self, maze: &Maze) {
        let all_faces = Face::get_all();

        let to_draw: Vec<Face> = all_faces.iter()
            .filter(|e| {
                let n = maze.get_neighbour(self, e);
                if n.is_none() { return true; }

                let n = n.unwrap();
                let faces = n.get_faces();

                let has_all = faces.len() == Face::get_all().len();

                let has_face = faces.iter()
                    .any(|x| x == *e);

                let has_opposite = faces
                    .iter()
                    .any(|x| x.opposite() == **e);

                if has_opposite && !has_all { return false; }

                if has_face { return false; }


                return true;
            })
            .map(|e| e.to_owned())
            .collect();

            let mut s = self.to_draw_cache.write().unwrap();
            *s = to_draw;

            drop(s);
    }

    pub fn get_faces(&self) -> Vec<Face> {
        return self.faces.clone();
    }

    ///! IMPORTANT NOTICE: USE update_cache after using this method
    pub fn add_face(&mut self, face: Face) {
        if !self.faces.contains(&face) {
            self.faces.push(face);
        }
    }

    pub fn get_pos(&self) -> Vec2 {
        return Vec2 { x: self.x, y: self.y };
    }

    pub fn draw(&self, ctx: &mut PaintCtx, scale: f64, centered: f64) {
        let x_start = self.x * scale;
        let y_start = self.y * scale;


        ctx.fill(Circle::new((x_start + centered, y_start + centered), POINT_RADIUS), &self.color);

        let to_draw = self.to_draw_cache.read().unwrap().clone();

        for side in to_draw.iter() {
            //side.draw(&Color::WHITE, ctx, scale, x_start, y_start);
        }
    }
}