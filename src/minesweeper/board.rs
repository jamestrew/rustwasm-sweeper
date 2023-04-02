use super::cell::Cell;
use std::{error::Error, fmt::Display};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub enum BoardError {
    SetCellError(String),
}

impl Error for BoardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match self {
            BoardError::SetCellError(_) => todo!(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for BoardError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Board(Vec<Vec<Cell>>);

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        let board = (0..height)
            .map(|_| (0..width).map(|_| Cell::Closed).collect())
            .collect();
        Board(board)
    }

    pub fn get(&self, pos: &Pos) -> Option<&Cell> {
        match self.0.get(pos.row) {
            Some(row) => row.get(pos.col),
            None => None,
        }
    }

    pub fn set(&mut self, pos: &Pos, cell: Cell) -> Result<(), BoardError> {
        let err_msg = format!("Failed to set {:?} at {:?}", cell, pos);
        let err = BoardError::SetCellError(err_msg);
        match self.0.get_mut(pos.row) {
            Some(row) => {
                let c = row.get_mut(pos.col).ok_or(err)?;
                *c = cell;
            }
            None => return Err(err),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_board(actual: Board, expect: Vec<Vec<Cell>>) {
        assert_eq!(expect.len(), actual.0.len());
        for i in 0..expect.len() {
            assert_eq!(expect[i].len(), actual.0[i].len());
            for j in 0..expect[i].len() {
                assert_eq!(expect[i][j], actual.0[i][j]);
            }
        }
    }

    #[test]
    fn init_board() {
        let b = Board::new(4, 3);
        let expect = vec![
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
        ];
        assert_board(b, expect);
    }
}
