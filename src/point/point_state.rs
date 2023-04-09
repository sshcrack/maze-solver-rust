#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointState {
    Highlight,
    HighlightSecondary,
    SolvePath,
    Passage,
    Wall
}