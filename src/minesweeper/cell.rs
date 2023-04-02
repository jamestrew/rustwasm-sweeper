#[derive(Debug, PartialEq)]
pub enum Cell {
    Open(usize),
    Closed,
    Flagged,
    Mine,
}
