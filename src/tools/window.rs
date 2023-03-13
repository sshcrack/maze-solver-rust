use minifb::{Window, WindowOptions};

use super::consts::get_window_size;

pub fn setup_window() -> anyhow::Result<Window> {
    let w_size = get_window_size()?;
    let window = Window::new("Maze", w_size, w_size, WindowOptions::default())?;

    Ok(window)
}