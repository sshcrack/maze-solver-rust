use std::cmp::Ordering;

#[derive(Clone, Debug, Copy, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize
}


impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y) == (other.x, other.y)
    }
}

