use druid::{Color, PaintCtx, RenderContext, Vec2, kurbo::Line, Rect};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Face {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

impl Face {
    pub fn get_all() -> Vec<Face> {
        return vec![Face::DOWN, Face::LEFT, Face::RIGHT, Face::DOWN];
    }

    pub fn to_vec(&self) -> Vec2 {
        match self {
            Face::UP => Vec2::new(0.0, -1.0),
            Face::LEFT => Vec2::new(-1.0, 0.0),
            Face::RIGHT => Vec2::new(1.0, 0.0),
            Face::DOWN => Vec2::new(0.0, 1.0),
        }
    }

    pub fn opposite(&self) -> Face {
        match self {
            Face::UP => Face::DOWN,
            Face::LEFT => Face::RIGHT,
            Face::RIGHT => Face::LEFT,
            Face::DOWN => Face::UP,
        }
    }

    pub fn draw(&self, c: &Color, ctx: &mut PaintCtx, scale: f64, x_start: f64, y_start: f64) {
        let half_scale = scale / 2.0;
        let quarter_scale = scale / 4.0;

        let dir_vec = self.to_vec() * -1.0;
        let mult_vec = self.to_vec() * half_scale;

        let x_dir = mult_vec.x;
        let y_dir = mult_vec.y;

        let mut start = mult_vec.clone();
        let mut end = mult_vec.clone();

        if y_dir == 0.0 {
            start.y -= quarter_scale;
            end.y += quarter_scale;
        }

        if x_dir == 0.0 {
            start.x -= quarter_scale;
            end.x += quarter_scale;
        }

        let x_line_start = x_start + half_scale + start.x;
        let y_line_start = y_start + half_scale + start.y;

        let x_line_end = x_start + half_scale + end.x;
        let y_line_end = y_start + half_scale + end.y;

        ctx.stroke(Line::new((x_line_start, y_line_start), (x_line_end, y_line_end)), c, 1.0);
}
}
