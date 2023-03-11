use std::cmp::min;

use druid::PaintCtx;

use crate::math;

pub fn get_size(ctx: &PaintCtx) -> f64 {
    let size = ctx.size();
    if size.aspect_ratio() != 1.0 {
        eprintln!("Invalid aspect ratio of maze");
    }

    if size.width > size.height {
        return size.height;
    } else {
        return size.width;
    }
}
