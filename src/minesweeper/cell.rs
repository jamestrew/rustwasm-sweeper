#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Open(usize),
    Closed,
    Flagged,
    Mine,
}
