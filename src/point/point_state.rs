#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointState {
    VISITED,
    PASSAGE,
    WALL
}