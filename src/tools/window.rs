use egui::Color32;

use crate::{point::point_state::{PointState, VisualIndicator}, manager::Window};

use super::{consts::{Maze, MazeOptions, get_options, FRAME_COUNT}, math::vec2_to_numb};

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn update_maze(window: &Window, maze: &Maze, intended_wait: bool) -> anyhow::Result<()> {
    update_maze_debug(window, maze, &Vec::new(), intended_wait)
}

pub fn update_maze_debug(window: &Window, maze: &Maze, visual_overwrites: &Vec<Option<VisualIndicator>>, intended_wait: bool) -> anyhow::Result<()> {
    let MazeOptions { speed, show_animation ,.. } = get_options()?;
    if !show_animation { return Ok(()); }

    let mut s = FRAME_COUNT.write().unwrap();
    *s += 1;
    let count = s.clone();

    drop(s);
    if count % speed as u128 != 0 && !intended_wait { return Ok(()); }

    draw_maze_overwrites(window, maze, visual_overwrites)?;
    Ok(())
}

fn draw_maze_overwrites(window: &Window, maze: &Maze, visual_overwrites: &Vec<Option<VisualIndicator>>) -> anyhow::Result<()> {
    let MazeOptions { size, .. } = get_options()?;
    let buf_size = window.get_size();

    let scale = buf_size / size;
    let mut buffer = vec![Color32::BLACK; buf_size * buf_size];

    for pos in 0..maze.len() {
        let x = pos % size;
        let y = pos / size;

        let point = *maze.get(pos).unwrap();
        let overwrite = visual_overwrites.get(pos).unwrap_or(&None);
        let color = obtain_color(&point, overwrite);


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

    window.set_pixels(buffer);
    Ok(())
}

fn obtain_color(point: &PointState, overwrite: &Option<VisualIndicator>) -> Color32 {
    if overwrite.is_some() {
        let overwrite = overwrite.unwrap();
        match overwrite {
            VisualIndicator::SolvePath => Color32::from_rgb(128, 0, 255),
            VisualIndicator::Searching => Color32::from_rgb(0, 0, 255),
            VisualIndicator::Match => Color32::from_rgb(255, 0, 255),
            VisualIndicator::End => Color32::from_rgb(0, 255, 0),
            VisualIndicator::Start => Color32::from_rgb(255, 0, 0),
            VisualIndicator::Custom(c) => c
        }
    } else {
        match point {
            PointState::Passage => Color32::from_rgb(255, 255, 255),
            PointState::Wall => Color32::from_rgb(0, 0, 0),
        }
    }
}