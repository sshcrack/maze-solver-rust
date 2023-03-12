// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use std::sync::{Arc, RwLock};
use std::thread;

use consts::{DEFAULT_DIMENSION, MAZE_UPDATE};
use druid::piet::d2d::Bitmap;
use druid::piet::{FontFamily, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::{prelude::*, AspectRatioBox, CrossAxisAlignment, Flex, FlexParams};
use druid::{AppLauncher, Color, LocalizedString, Rect, WindowDesc};
use path::maze::Maze;
use rand::Rng;
use tool::size::get_size;

mod consts;
mod generator;
mod math;
mod path;
mod tool;

struct MazeDisplay {
    render_cache: Arc<RwLock<Option<Bitmap>>>,
}

impl MazeDisplay {
    fn mark_dirty(&self) {
        println!("Marking dirty");
        let mut s = self.render_cache.write().unwrap();
        *s = None;
        drop(s);
    }
}

impl Widget<Option<Maze>> for MazeDisplay {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut Option<Maze>, _env: &Env) {
        match event {
            Event::AnimFrame(_) => {
                ctx.request_paint();
                ctx.request_anim_frame();
            }
            Event::WindowConnected => {
                ctx.request_anim_frame();
                //ctx.request_timer(Duration::from_millis(UPDATE_INTERVAL));
            },
            Event::WindowScale(_) => {
                self.mark_dirty();
            },
            Event::WindowSize(_) => {
                self.mark_dirty();
            }
            Event::Command(e) => {
                if e.is(MAZE_UPDATE) {
                    let x = e.get(MAZE_UPDATE).unwrap();
                    *_data = x.to_owned();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Option<Maze>,
        _env: &Env,
    ) {
        ctx.request_paint();
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &Option<Maze>,
        _data: &Option<Maze>,
        _env: &Env,
    ) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Option<Maze>,
        _env: &Env,
    ) -> Size {
        self.mark_dirty();
        let e = if bc.is_width_bounded() || bc.is_height_bounded() {
            let max = bc.max();
            let dim = if max.height < max.width {
                max.height
            } else {
                max.width
            };

            bc.constrain_aspect_ratio(1.0, dim)
        } else {
            bc.constrain_aspect_ratio(1.0, 900.0)
        };

        return e;
    }

    fn paint(&mut self, ctx: &mut PaintCtx, maze: &Option<Maze>, _env: &Env) {
        let size = get_size(ctx);

        if maze.is_none() {
            ctx.fill(Rect::new(0.0, 0.0, size, size), &Color::BLACK);
            let text = ctx.text();
            let layout = text
                .new_text_layout("Generating maze...")
                .font(FontFamily::SERIF, 24.0)
                .text_color(Color::rgb8(128, 0, 0))
                .build()
                .unwrap();
            ctx.draw_text(&layout, (100.0, 25.0));
            let mut s = self.render_cache.write().unwrap();
            *s = None;
            drop(s);
        } else {
            let maze = maze.as_ref().unwrap();

            if self.render_cache.read().unwrap().is_none() {
                ctx.fill(Rect::new(0.0, 0.0, size, size), &Color::BLACK);
                maze.mark_dirty();

                maze.draw(ctx);

                let e = ctx
                    .capture_image_area(Rect::new(0.0, 0.0, size, size))
                    .unwrap();
                let mut s = self.render_cache.write().unwrap();
                *s = Some(e);
                drop(s);
            } else {
                let s = self.render_cache.read().unwrap();
                let e = s.as_ref().unwrap();
                ctx.draw_image(
                    &e,
                    Rect::new(0.0, 0.0, size, size),
                    InterpolationMode::Bilinear,
                );

                drop(s);
            }
        }
    }
}

pub fn main() {
    let mut col = Flex::row();
    let display = AspectRatioBox::new(
        MazeDisplay {
            render_cache: Arc::new(RwLock::new(None)),
        },
        1.0,
    );
    let mut maze = Maze::create(DEFAULT_DIMENSION);

    col.add_flex_child(display, FlexParams::new(1.0, CrossAxisAlignment::End));
    let window = WindowDesc::new(col)
        .title(LocalizedString::new("maze-title").with_placeholder("Maze Generator"))
        .resizable(true)
        .window_size_policy(druid::WindowSizePolicy::User);

    let launcher = AppLauncher::with_window(window).log_to_console();

    let sink = launcher.get_external_handle();
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            for ele in maze.get_all_mut() {
                let r = rng.gen::<f64>();
                let g = rng.gen::<f64>();
                let b = rng.gen::<f64>();
                ele.color = Color::rgb(r, g, b);
            }

            let temp = maze.clone();
            sink.add_idle_callback(move |opt: &mut Option<Maze>| *opt = Some(temp));
            //thread::sleep(Duration::from_millis(2000));
        }
    });

    launcher.launch(None).expect("launch failed");
}
