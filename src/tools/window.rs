use minifb::{Window, WindowOptions};

use crate::point::point_state::PointState;

use super::{consts::{get_window_size, get_size, get_scale, Maze}, math::vec2_to_numb};

pub fn setup_window() -> anyhow::Result<Window> {
    let w_size = get_window_size()?;
    let window = Window::new("Maze", w_size, w_size, WindowOptions::default())?;

    Ok(window)
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn draw_maze(window: &mut Window, maze: &Maze) -> anyhow::Result<()> {
    let size = get_size()?;
    let scale = get_scale()?;

    let buf_size = size * scale;
    let mut buffer = vec![0; buf_size * buf_size];

    for pos in 0..maze.len() {
        let x = pos % size;
        let y = pos / size;

        let point = maze.get(pos).unwrap();
        let color = match point {
            PointState::PASSAGE => from_u8_rgb(255, 0, 0),
            PointState::VISITED=> from_u8_rgb(0, 255, 0),
            PointState::WALL => from_u8_rgb(0, 0, 0)
        };

        let rel_x = ((x as f64) / (size as f64) * buf_size as f64) as usize;
        let rel_y = ((y as f64) / (size as f64) * buf_size as f64) as usize;
        for x_chunk in 0..scale {
            for y_chunk in 0..scale {
                let one_d = vec2_to_numb(rel_x + x_chunk, rel_y + y_chunk, buf_size);
                let index = one_d;

                buffer[index] = color;
            }
        }
    }

    window.update_with_buffer(&buffer, buf_size, buf_size)?;
    Ok(())
}