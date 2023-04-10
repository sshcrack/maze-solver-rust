use std::{ops::Range, iter::StepBy};

use rand_distr::num_traits::Pow;

use crate::point::point::Point;

pub fn vec2_to_numb(x: usize, y: usize, size: usize) -> usize {
    return y * size + x;
}

pub fn numb_to_vec2(numb: usize, size: usize) -> (usize, usize) {
    return (numb % size, numb / size);
}

pub fn point_to_numb(p: &Point, size: usize) -> usize {
    return vec2_to_numb(p.x, p.y, size);
}

pub fn get_point<T>(matrix: &[T], Point {x, y}: &Point) -> T
    where
        T: Copy
{
    let size = (matrix.len() as f64).sqrt() as usize;
    let pos = vec2_to_numb(*x, *y, size);

    matrix[pos]
}

pub fn set_point<T>(matrix: &mut [T], Point {x, y}: &Point, state: T) {
    let size = (matrix.len() as f64).sqrt() as usize;
    matrix[vec2_to_numb(*x, *y, size)] = state;
}

pub fn set_point_mult<T>(matrix: &mut [T], points: &Vec<Point>, state: T)
    where T: Clone + Copy
{
    for p in points {
        set_point(matrix, p, state.clone());
    }
}


pub fn get_maze_iter(size: &usize) -> StepBy<Range<usize>> {
    (1..size.clone()).step_by(2)
}

pub fn get_dist(src: &Point, dest: &Point) -> u64 {
    let Point { x: s_x, y: s_y} = src;
    let Point { x: d_x, y: d_y} = dest;

    let s_x = *s_x as i32;
    let s_y = *s_y as i32;

    let d_x = *d_x as i32;
    let d_y = *d_y as i32;

    return ((s_x - d_x).abs() + (s_y - d_y).abs()).try_into().unwrap()
}

pub fn linear_dist(src: &Point, dest: &Point) -> f64 {
    let Point { x: s_x, y: s_y} = src;
    let Point { x: d_x, y: d_y} = dest;

    let s_x = *s_x as f64;
    let s_y = *s_y as f64;

    let d_x = *d_x as f64;
    let d_y = *d_y as f64;

    let out: f64 = (d_x - s_x).pow(2) + (d_y - s_y).pow(2);
    return out.sqrt()
}