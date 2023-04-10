use std::collections::BinaryHeap;

use anyhow::{Result, anyhow};
use egui::Color32;

use crate::{tools::{consts::{Maze, get_size}, matrix::{go_to_dir, get_available_dirs_state, has_passage_between, get_pos_between}, math::{set_point, point_to_numb, linear_dist}, window::{update_maze_debug, from_u8_rgb}}, solve::solve::SolveOptions, point::{point_state::{VisualIndicator, PointState}, point::Point}, manager::Window};
use super::Node;

pub fn a_star(maze: &mut Maze, window: &Window, options: &SolveOptions) -> Result<Vec<Point>> {
    println!("Running a*");
    let SolveOptions { start, end, ..} = options;
    let size = get_size()?;

    let mut visual_overwrites = vec![None; size * size];
    let mut nodes = Vec::with_capacity(size * size);
    for x in 0..size {
        for y in 0..size {
            nodes.push(Node::new(Point { x, y }, &end));
        }
    }

    let start_index = point_to_numb(&start, size);
    let start_node = nodes.get_mut(start_index).unwrap();

    start_node.set_start_node();

    let start_cost = start_node.calculate_cost(&start_node);
    drop(start_node);

    let mut pending = BinaryHeap::new();

    set_point(&mut visual_overwrites, &start, Some(VisualIndicator::Start));
    set_point(&mut visual_overwrites, &end, Some(VisualIndicator::End));

    let mut found = false;
    pending.push(start.clone());

    let mut end_node = None;
    while !pending.is_empty() && !found {
        let pos = pending.pop().unwrap();
        let node = nodes.get(point_to_numb(&pos, size)).unwrap().clone();

        let dirs = get_available_dirs_state(maze, &pos, PointState::Passage)?;
        for dir in dirs {
            let neighbor = go_to_dir(&pos, &dir)?;
            if neighbor.is_none() { continue; }

            let between_pos = get_pos_between(&pos, &dir)?.unwrap();
            let has_passage = has_passage_between(maze, &pos, &dir)?;

            let has_passage = has_passage.unwrap_or(false);
            if !has_passage { continue; }

            let neighbor_pos = neighbor.unwrap();

            let neighbor_index = point_to_numb(&neighbor_pos, size);
            let neighbor = nodes.get_mut(neighbor_index).unwrap();

            let temp_cost = neighbor.calculate_cost(&node);
            if temp_cost < neighbor.get_cost() {
                let color = ((1.0 - (linear_dist(&neighbor_pos, &end) as f64) / (start_cost as f64)) * 255.0).abs().floor() as u8;

                set_point(&mut visual_overwrites, &neighbor_pos, Some(VisualIndicator::Custom(Color32::from_rgb(color, 0, 255))));
                set_point(&mut visual_overwrites, &between_pos, Some(VisualIndicator::Custom(Color32::from_rgb(color, 0, 255))));
                neighbor.update(&node);
                pending.push(neighbor_pos);
            }

            if &neighbor_pos == end {
                println!("FOUND!");
                found = true;

                end_node = Some(neighbor_index);
                set_point(&mut visual_overwrites, &end, Some(VisualIndicator::End));
                for _ in 0..100 {
                    update_maze_debug(window, maze, &visual_overwrites, false)?;
                }
                break;
            }
        }

        for _ in 0..2 {
            update_maze_debug(window, maze, &visual_overwrites, false)?;
        }
    }

    if end_node.is_none() {
        return Err(anyhow!("Could not solve maze."))
    }


    let end_node = nodes.get(end_node.unwrap()).unwrap();
    Ok(node_to_path(end_node, start))
}

fn node_to_path(node: &Node, start: &Point) -> Vec<Point> {
    let mut path = Vec::new();
    let mut curr_node = node.clone();

    while curr_node.get_parent().is_some() {
        path.push(curr_node.get_pos());

        curr_node = curr_node.get_parent().clone().unwrap();
    }

    path.push(start.clone());
    path.reverse();
    return path;
}