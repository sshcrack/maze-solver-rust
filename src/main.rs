#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use egui::*;
use manager::{MazeThread, Window};

mod generators;
mod manager;
mod point;
mod solve;
mod tools;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Maze Solver",
        options,
        Box::new(|cc| {
            let ctx = cc.egui_ctx.clone();
            let app = MyApp::new(&ctx);

            let temp = app.size.clone();
            let temp1 = app.pixels.clone();
            thread::spawn(|| maze_thread(ctx, temp, temp1));
            Box::new(app)
        }),
    )
}

fn maze_thread(ctx: Context, size_arc: Arc<RwLock<usize>>, maze: Arc<RwLock<Vec<Color32>>>) {
    loop {
        let size = size_arc.read().unwrap().clone();
        let mut s = maze.write().unwrap();
        *s = vec![Color32::BLACK; size * size];


        let e = rand::random();
        for x in 0..size {
            for y in 0..size {
                s[size * y + x] = Color32::from_rgb(e, e, e)
            }
        }
        drop(s);
        ctx.request_repaint();
        thread::sleep(Duration::from_millis(100));
    }
}

struct MyApp {
    pixels: Arc<RwLock<Vec<Color32>>>,
    size: Arc<RwLock<usize>>,
    curr: MazeThread
}

impl MyApp {
    fn new(ctx: &Context) -> Self {
        let pixels = Arc::new(RwLock::new(Vec::new()));
        let size = Arc::new(RwLock::new(0));
        let window = Window::new(&ctx, &size, &pixels);

        Self {
            pixels,
            size,
            curr: MazeThread::new_default(&window)
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                let mut t = "".to_string();
                ui.text_edit_singleline(&mut t).labelled_by(name_label.id);
            });
            ui.label(format!("Seed is {}", ""));

            let mut texture =
                ctx.load_texture("maze-texture", ColorImage::example(), Default::default());

            let left = ui.available_size_before_wrap();
            let size = left.min_elem() as usize;
            *self.size.write().unwrap() = size;

            let mut img = ColorImage::new([size, size], Color32::BLACK);

            let to_fill = self.pixels.read().unwrap().clone();
            for i in 0..to_fill.len().min(img.pixels.len()) {
                img.pixels[i] = to_fill[i];
            }

            texture.set(img, Default::default());
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.image(texture.id(), texture.size_vec2())
            });
        });
    }
}
