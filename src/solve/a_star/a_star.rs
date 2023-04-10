use std::{collections::BinaryHeap, cmp::max};

use anyhow::{Result, anyhow};
use minifb::Window;

use crate::{tools::{consts::{Maze, get_size}, matrix::{go_to_dir, get_available_dirs_state, has_passage_between}, math::{set_point, point_to_numb, get_dist}, window::{update_maze_debug, from_u8_rgb}}, solve::solve::SolveOptions, point::point_state::{VisualIndicator, PointState}};
use super::Node;

pub fn a_star(maze: &mut Maze, window: &mut Window, options: &SolveOptions) -> Result<()> {
    println!("Running a*");
    let SolveOptions { start, end, ..} = options;
    let size = get_size()?;

    let mut visual_overwrites = vec![None; size * size];
    let mut f_score = vec![u64::MAX -1; size * size];
    let mut g_score = vec![u64::MAX -1; size * size];
    let mut nodes = vec![Node::default(); size * size];

    let mut pending = BinaryHeap::new();
    let start_node = Node::new(start, end, None);


    set_point(&mut visual_overwrites, &start, Some(VisualIndicator::Start));
    set_point(&mut visual_overwrites, &end, Some(VisualIndicator::End));
    let start_cost = start_node.get_cost();

    let mut found = false;
    pending.push(start);

    g_score[point_to_numb(&start, size)] = 0;
    f_score[point_to_numb(&start, size)] = get_dist(&start, &end);

    while !pending.is_empty() && !found {
        let pos = pending.pop().unwrap();
        let node = nodes[point_to_numb(pos, size)];

        let dirs = get_available_dirs_state(maze, &pos, PointState::Passage)?;
        for dir in dirs {
            let neighbor = go_to_dir(&pos, &dir)?;
            if neighbor.is_none() { continue; }

            let has_passage = has_passage_between(maze, &pos, &dir)?;

            let has_passage = has_passage.unwrap_or(false);
            if !has_passage { continue; }

            let neighbor = neighbor.unwrap();

            let n = Node::new(&neighbor, end, Some(&node));
            if &neighbor == end {
                println!("FOUND!");
                found = true;
                break;
            }

            let temp_g_score = g_score[point_to_numb(&pos, size)] +1;
            let temp_f_score = temp_g_score + get_dist(&neighbor, end);

            if temp_f_score < f_score[point_to_numb(&neighbor, size)] {
                f_score[point_to_numb(&neighbor, size)] = temp_f_score;
                g_score[point_to_numb(&neighbor, size)] = temp_g_score;
                pending.push(neighbor);
            }
        }

        let relative = (255 * node.get_cost() / start_cost) as u8;
        let color = from_u8_rgb(relative, 0, relative);
        set_point(&mut visual_overwrites, &node.get_pos(), Some(VisualIndicator::Custom(color)));

        for _ in 0..10 {
            update_maze_debug(window, maze, &visual_overwrites, false)?;
        }
    }

    loop {}
    Ok(())
}

