use std::sync::{Arc, Mutex, RwLock};

use druid::{Color, PaintCtx, RenderContext, Vec2, kurbo::Circle, Rect};

use crate::{consts::POINT_RADIUS, path::maze::Maze};

use super::face::Face;

#[derive(Clone)]
pub struct MazePoint {
    pub color: Color,
    faces: Vec<Face>,
    x: f64,
    y: f64,
    is_dirty: Arc<RwLock<bool>>
}

impl MazePoint {
    pub fn default(x: f64, y: f64) -> Self {
        return Self {
            color: Color::RED,
            faces: vec![Face::LEFT],
            x,
            y,
            is_dirty: Arc::new(RwLock::new(true))
        };
    }

    pub fn get_faces(&self) -> Vec<Face> {
        return self.faces.clone();
    }

    ///! IMPORTANT NOTICE: USE update_cache after using this method
    pub fn add_face(&mut self, face: Face) {
        if !self.faces.contains(&face) {
            self.faces.push(face);
        }
        let mut s = self.is_dirty.write().unwrap();
        *s = true;
        drop(s);
    }

    pub fn get_pos(&self) -> Vec2 {
        return Vec2 { x: self.x, y: self.y };
    }

    pub fn draw(&self, ctx: &mut PaintCtx, scale: f64, centered: f64, maze: &Maze) {
        if !*self.is_dirty.read().unwrap() {
            return;
        }

        let x_start = self.x * scale;
        let y_start = self.y * scale;

        ctx.fill(Rect::new(x_start, y_start, x_start + scale, y_start + scale), &Color::BLACK);
        ctx.fill(Circle::new((x_start + centered, y_start + centered), POINT_RADIUS), &self.color);

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

        for side in to_draw.iter() {
            side.draw(&Color::WHITE, ctx, scale, x_start, y_start);
        }

        let mut s = self.is_dirty.write().unwrap();
        *s = false;
        drop(s);
    }

    pub fn mark_dirty(&self) {
        let mut s = self.is_dirty.write().unwrap();
        *s = true;
        drop(s);
    }
}