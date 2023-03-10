use druid::{Color, PaintCtx, RenderContext, kurbo::Shape, Rect, piet::Brush};

use crate::consts::RES;

use super::face::Face;

pub struct MazePoint {
    pub color: Color,
    pub faces: Vec<Face>,
    x: usize,
    y: usize
}

impl MazePoint {
    pub fn default(x: usize, y: usize) -> Self {
        return Self {
            color: Color::RED,
            faces: Face::get_all(),
            x,
            y
        }
    }

    pub fn draw(&self, ctx: &mut PaintCtx) {
        let x0 = self.x as f64 * RES;
        let x1 = x0 + RES;

        let y0 = self.y as f64 * RES;
        let y1 = y0 + RES;

        ctx.fill(Rect::new(x0, y0, x1, y1), &self.color)
    }
}