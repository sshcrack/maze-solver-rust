#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    sync::{Arc, RwLock}
};

use eframe::{App, Theme};
use egui::*;
use manager::{MazeThread, Window};
use solve::solve::SolveAlgorithm;
use tools::consts::MazeOptions;

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
            Box::new(MyApp::new(&cc.egui_ctx))
        }),
    )
}

struct MyApp {
    pixels: Arc<RwLock<Vec<Color32>>>,
    pixels_size: Arc<RwLock<usize>>,
    maze_img: Option<TextureHandle>,
    curr: Option<MazeThread>,

    seed_random: bool,
    seed: u64,
    seed_text: String,

    solve_algorithm: SolveAlgorithm,
    size: usize,
    size_text: String,

    window: Window,

    show_animation: bool,

    speed: f64,
    speed_text: String
}

impl MyApp {
    fn new(ctx: &Context) -> Self {
        let pixels = Arc::new(RwLock::new(Vec::new()));
        let pixels_size = Arc::new(RwLock::new(0));
        let should_exit = Arc::new(RwLock::new(false));
        let window = Window::new(&ctx, &pixels_size, &pixels, &should_exit);

        let rand_seed = rand::random();
        Self {
            pixels,
            maze_img: None,
            seed: rand_seed,
            seed_text: rand_seed.to_string(),
            seed_random: true,
            pixels_size,
            curr: None,
            window,
            solve_algorithm: SolveAlgorithm::AStar,

            size: 50,
            size_text: "50".to_string(),

            show_animation: true,

            speed: 1.0,
            speed_text: "1.0".to_string()
        }
    }

    fn start_generating(&self) -> MazeThread {
        MazeThread::new(&self.window, &MazeOptions {
            seed: self.seed,
            show_animation: self.show_animation,
            size: self.size,
            speed: self.speed,

        }, self.solve_algorithm)
    }
}

impl MyApp {
    fn add_image(&mut self, ctx: &Context, ui: &mut Ui) {
        let left = ui.available_size_before_wrap();
        let size = left.min_elem() as usize;
        *self.pixels_size.write().unwrap() = size;

        let mut img = ColorImage::new([size, size], Color32::BLACK);

        let texture = self.maze_img.get_or_insert_with(|| {
            ctx.load_texture("maze-texture", ColorImage::example(), Default::default())
        });

        let to_fill = self.pixels.read().unwrap().clone();
        for i in 0..to_fill.len().min(img.pixels.len()) {
            img.pixels[i] = to_fill[i];
        }

        texture.set(img, Default::default());
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.image(texture.id(), texture.size_vec2())
        });
    }

    fn regenerate_seed(&mut self) {
        let r = rand::random();
        self.seed = r;
        self.seed_text = r.to_string();
    }

    fn add_seed_selector(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            let checkbox = ui.checkbox(&mut self.seed_random, "Random?");
            if checkbox.changed() {
                if self.seed_random { self.regenerate_seed() }
            }


            let seed_label = ui.label("Seed: ");
            ui.add_enabled_ui(!self.seed_random, |ui| {
                if self.seed_random {
                    self.seed_text = self.seed.to_string();
                }

                let valid_seed = self.seed_text.parse::<u64>().is_ok();
                let theme = frame.info().system_theme.unwrap_or(Theme::Dark);

                let mut text_color = if theme == Theme::Dark { Color32::WHITE } else { Color32::BLACK };
                if !valid_seed {
                    text_color = if theme == Theme::Dark { Color32::LIGHT_RED } else { Color32::DARK_RED };
                }

                let res = TextEdit::singleline(&mut self.seed_text)
                    .text_color(text_color)
                    .ui(ui)
                    .labelled_by(seed_label.id);
                if res.changed() {
                    let parse_res = self.seed_text.parse::<u64>();
                    if parse_res.is_ok() { self.seed = parse_res.unwrap(); }
                }
            });
        });
    }

    fn add_size_selector(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            let size_label = ui.label("Size: ");
            let valid_size = self.size_text.parse::<usize>().is_ok();
            let theme = frame.info().system_theme.unwrap_or(Theme::Dark);

            let mut text_color = if theme == Theme::Dark { Color32::WHITE } else { Color32::BLACK };
            if !valid_size {
                text_color = if theme == Theme::Dark { Color32::LIGHT_RED } else { Color32::DARK_RED };
            }

            let res = TextEdit::singleline(&mut self.size_text)
                .text_color(text_color)
                .ui(ui)
                .labelled_by(size_label.id);
            if res.changed() {
                let parse_res = self.size_text.parse::<usize>();
                if parse_res.is_ok() { self.size = parse_res.unwrap(); }
            }
        });
    }

    fn add_speed_selector(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            let speed_label = ui.label("Speed: ");
            let valid_speed = self.speed_text.parse::<f64>().is_ok();
            let theme = frame.info().system_theme.unwrap_or(Theme::Dark);

            let mut text_color = if theme == Theme::Dark { Color32::WHITE } else { Color32::BLACK };
            if !valid_speed {
                text_color = if theme == Theme::Dark { Color32::LIGHT_RED } else { Color32::DARK_RED };
            }

            let res = TextEdit::singleline(&mut self.speed_text)
                .text_color(text_color)
                .ui(ui)
                .labelled_by(speed_label.id);
            if res.changed() {
                let parse_res = self.speed_text.parse::<f64>();
                if parse_res.is_ok() { self.speed = parse_res.unwrap(); }
            }
        });
    }

    fn add_show_animation(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let size_label = ui.label("Show Animation: ");
            ui.checkbox(&mut self.show_animation, "Show Animation").labelled_by(size_label.id);
        });
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Maze Generator / Solver");
            self.add_seed_selector(ui, frame);
            self.add_size_selector(ui, frame);
            self.add_speed_selector(ui, frame);
            self.add_show_animation(ui);

            let mut text = "Generate";
            if self.curr.is_some() {
                let thread = self.curr.as_ref().unwrap();
                let signal_sent = thread.exit_signal_sent();

                if thread.is_finished() {
                    self.curr = None;
                }
                if signal_sent {
                    text = "Stopping...";
                } else {
                    text = "Stop";
                }
            }

            let res = ui.button(text);
            if res.clicked() {
                if self.curr.is_some() {
                    let thread = self.curr.as_ref().unwrap();
                    thread.terminate();
                    self.regenerate_seed();
                } else {
                    self.curr = Some(self.start_generating());
                }
            }

            self.add_image(ctx, ui);
        });
    }
}
