#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Pos {
    pub row: u8,
    pub col: u8,
}

impl Pos {
    pub fn key(&self) -> String {
        format!("{}{}", self.row, self.col)
    }
}
