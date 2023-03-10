// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock, mpsc, Mutex};
use std::thread;
use std::time::Duration;

use consts::{SIZE, DIMENSION};
use druid::widget::{prelude::*, Container};
use druid::{AppLauncher, Color, LocalizedString, WindowDesc, Rect, WindowSizePolicy};
use path::maze::Maze;
use rand::Rng;

mod path;
mod math;
mod generator;
mod consts;

struct MazeDisplay {
    maze: Arc<RwLock<Maze>>,
    rx: Receiver<()>
}

impl Widget<()> for MazeDisplay {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut (), _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                //self.maze = 0.0;
                ctx.request_anim_frame();
            }
            Event::AnimFrame(_) => {
                ctx.request_paint();
            },
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &(),
        _env: &Env,
    ) -> Size {
        let size = (SIZE as f64, SIZE as f64);
        println!("{:#?}", size);
        bc.constrain(size);

        return Size::new(SIZE as f64, SIZE as f64);
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        let state  = self.maze.read().unwrap();

        ctx.fill(Rect::new(0.0, 0.0, SIZE as f64, SIZE as f64), &Color::BLACK);
        state.draw(ctx);
    }
}

pub fn main() {
    let (tx, rx ) = mpsc::channel();
    let maze = Maze::create(DIMENSION, Arc::new(Mutex::new(tx)));
    let arc = Arc::new(RwLock::new(maze));

let e = arc.clone();
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            let mut state = e.write().unwrap();
            for ele in state.get_all_mut() {
                let r = rng.gen::<f64>();
                let g = rng.gen::<f64>();
                let b = rng.gen::<f64>();
                ele.color = Color::rgb(r, g, b)
            }
            drop(state);

            thread::sleep(Duration::from_secs(1))
        }
    });

    let col = Container::new(MazeDisplay {
        maze: arc,
        rx
    });


    let window = WindowDesc::new(col).title(
        LocalizedString::new("maze-title")
            .with_placeholder("Maze Generator"),
    )
    .resizable(false)
    .window_size_policy(WindowSizePolicy::Content);

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(())
        .expect("launch failed");
}