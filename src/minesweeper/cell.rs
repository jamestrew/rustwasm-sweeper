#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Open(u8),
    Closed,
    Flagged,
    Mine,
}
