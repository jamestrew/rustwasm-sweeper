use super::cell::Cell;
use std::{error::Error, fmt::Display};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub enum BoardError {
    SetCellError { pos: Pos, cell: Cell },
}

impl Error for BoardError {}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::SetCellError { pos, cell } => {
                write!(f, "failed to set {:?} at {:?}", cell, pos)
            }
        }
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

    pub fn get(&self, pos: Pos) -> Option<&Cell> {
        match self.0.get(pos.row) {
            Some(row) => row.get(pos.col),
            None => None,
        }
    }

    pub fn set(&mut self, pos: Pos, cell: Cell) -> Result<(), BoardError> {
        let err = BoardError::SetCellError { pos, cell };
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

    #[test]
    fn board_get_inbounds() {
        let mut b = Board::new(4, 3);
        b.0[0][1] = Cell::Mine;
        assert_eq!(b.get(Pos { row: 0, col: 1 }).unwrap(), &Cell::Mine);
    }

    #[test]
    fn board_get_outbounds() {
        let b = Board::new(4, 3);
        assert_eq!(b.get(Pos { row: 10, col: 1 }), None);
    }

    #[test]
    fn board_set_inbounds() {
        let mut b = Board::new(4, 3);
        let pos = Pos { row: 0, col: 0 };
        assert_eq!(b.get(pos).unwrap(), &Cell::Closed);
        assert_eq!(b.set(pos, Cell::Mine).unwrap(), ());
        assert_eq!(b.get(pos).unwrap(), &Cell::Mine);
    }

    #[test]
    fn board_set_outbounds() {
        let mut b = Board::new(4, 3);
        let pos = Pos { row: 100, col: 0 };
        let res = b.set(pos, Cell::Mine);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            BoardError::SetCellError {
                pos,
                cell: Cell::Mine
            }
        );
    }
}
