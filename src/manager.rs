use std::{
    thread::{self, JoinHandle},
};

use anyhow::Result;
use image::{RgbaImage, ImageBuffer, Rgba, ImageFormat};

use crate::{
    generators::generate::generate,
    point::{point::Point, point_state::VisualIndicator},
    solve::solve::{solve, SolveAlgorithm, SolveOptions},
    tools::{
        consts::{get_size, check_size, MazeOptions},
        math::{set_point, set_point_mult, points_to_dir, vec2_to_numb},
        matrix::get_pos_between,
        window::{update_maze_overwrite, update_maze_debug_overwrite}, options::MazeData,
    },
};

pub struct MazeThread {
    data: MazeData,
    thread: JoinHandle<Result<()>>
}

impl MazeThread {
    pub fn is_finished(&self) -> bool {
        self.thread.is_finished()
    }

    pub fn exit_signal_sent(&self) -> bool {
        self.data.should_exit()
    }

    pub fn get_options(&self) -> MazeOptions {
        return self.data.get_opt()
    }

    #[allow(dead_code)]
    pub fn get_data(&self) -> &MazeData {
        &self.data
    }

    pub fn get_mut_data(&mut self) -> &mut MazeData {
        &mut self.data
    }

    pub fn terminate(&self) {
        if self.exit_signal_sent() {
            return;
        }

        self.data.set_should_exit(true);
    }

    pub fn new(data: &MazeData, algorithm: SolveAlgorithm) -> Self {
        data.should_exit();

        let temp = data.clone();
        Self {
            data: data.clone(),
            thread: thread::spawn(move || MazeThread::main_run(temp, algorithm)),
        }
    }

    pub fn main_run(data: MazeData, algorithm: SolveAlgorithm) -> Result<()> {
        check_size(&data);
        let end_coords = get_size(&data)? - 2;

        println!("Generating...");
        let mut maze = generate(&data)?;

        let start = Point { x: 1, y: 1 };
        let end = Point {
            x: end_coords,
            y: end_coords,
        };

        let options = SolveOptions {
            algorithm,
            start,
            end,
        };

        let size = get_size(&data)?;
        println!("Solving...");
        let path = solve(&mut maze, &data, &options)?;
        let mut visual_overwrites = vec![None as Option<VisualIndicator>; size * size];

        println!("Drawing...");
        for i in 0..path.len() {
            let next_index = i + 1;
            if next_index == path.len() {
                continue;
            }

            let p = path[i];
            let n = path[next_index];

            let dir = points_to_dir(&n, &p);
            if dir.is_none() {
                eprintln!("Could not find direction for {:?} to {:?}6", p, n);
                continue;
            }

            let between = get_pos_between(&data, &p, &dir.unwrap())?.unwrap();
            set_point_mult(
                &mut visual_overwrites,
                &vec![p, between, n],
                Some(VisualIndicator::SolvePath),
            );

            if data.show_debug() {
                print!("{}", p);
                if i != path.len() -2 { print!(" -> "); }
            }
        }
        println!("");

        set_point(&mut visual_overwrites, &start, Some(VisualIndicator::Start));
        set_point(&mut visual_overwrites, &end, Some(VisualIndicator::End));

        data.set_done(true);
        while !data.should_exit() {
            if data.show_debug() {
                update_maze_debug_overwrite(&data, &maze, &visual_overwrites, true, true)?;
            } else {
                update_maze_overwrite(&data, &maze, true)?;
            }

            let save_path = data.take_requested();
            if save_path.is_some() {
                let save_path = save_path.unwrap();
                let pixels = data.get_pixels();

                let mut out: RgbaImage = ImageBuffer::new(size as u32, size as u32);
                for pixel in out.enumerate_pixels_mut() {
                    let index = vec2_to_numb(pixel.0 as usize, pixel.1 as usize, size);
                    *pixel.2 = Rgba(pixels[index].to_array());
                }

                out.save_with_format(save_path, ImageFormat::Png).unwrap();
            }
        }

        data.request_repaint();
        data.set_gen_proc(1.0);
        println!("Done.");
        Ok(())
    }
}