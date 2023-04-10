#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointState {
    Passage,
    Wall
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisualIndicator {
    Searching,
    Match,
    SolvePath,
    Start,
    End,
    Custom(u32)
}