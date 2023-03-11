use druid::Selector;

use crate::path::maze::Maze;

pub const DEFAULT_DIMENSION: usize = 60;
pub const MAZE_UPDATE: Selector<Option<Maze>> = Selector::new("maze_gen.maze_update");
pub const UPDATE_INTERVAL: u64 = 10;
pub const POINT_RADIUS: f64 = 2.5;