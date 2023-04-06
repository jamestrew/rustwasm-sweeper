use super::cell::Cell;
use std::{error::Error, fmt::Display};


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Pos {
    pub row: u8,
    pub col: u8,
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

pub struct Board {
    b: Vec<Vec<Cell>>,
    pub height: u8,
    pub width: u8,
}

impl Board {
    pub fn new(height: u8, width: u8) -> Self {
        let board = (0..height)
            .map(|_| (0..width).map(|_| Cell::Closed).collect())
            .collect();

        Board {
            b: board,
            height,
            width,
        }
    }

    pub fn get(&self, pos: Pos) -> Option<&Cell> {
        match self.b.get(pos.row as usize) {
            Some(row) => row.get(pos.col as usize),
            None => None,
        }
    }

    pub fn set(&mut self, pos: Pos, cell: Cell) -> Result<(), BoardError> {
        let err = BoardError::SetCellError { pos, cell };
        match self.b.get_mut(pos.row as usize) {
            Some(row) => {
                let c = row.get_mut(pos.col as usize).ok_or(err)?;
                *c = cell;
            }
            None => return Err(err),
        };

        Ok(())
    }

    pub fn iter_neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        let height = self.height;
        let width = self.width;

        let row_start = if pos.row > 0 { pos.row - 1 } else { pos.row };
        let row_end = if pos.row < height - 1 {
            pos.row + 1
        } else {
            pos.row
        };
        let col_start = if pos.col > 0 { pos.col - 1 } else { pos.col };
        let col_end = if pos.col < width - 1 {
            pos.col + 1
        } else {
            pos.col
        };

        let cell_row = pos.row;
        let cell_col = pos.col;
        (row_start..=row_end).flat_map(move |row| {
            (col_start..=col_end).filter_map(move |col| {
                if row == cell_row && col == cell_col {
                    None
                } else {
                    Some(Pos { row, col })
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn assert_board(actual: Board, expect: Vec<Vec<Cell>>) {
        assert_eq!(expect.len(), actual.height as usize);
        for i in 0..expect.len() {
            assert_eq!(expect[i].len(), actual.width as usize);
            for j in 0..expect[i].len() {
                assert_eq!(expect[i][j], actual.b[i][j]);
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
        b.b[0][1] = Cell::Mine;
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

    #[test]
    fn board_corner_iter_neighbors() {
        let b = Board::new(4, 3);
        let ret: HashSet<_> = b.iter_neighbors(Pos { row: 0, col: 0 }).collect();
        let expect = HashSet::from([
            Pos { row: 0, col: 1 },
            Pos { row: 1, col: 0 },
            Pos { row: 1, col: 1 },
        ]);
        assert_eq!(ret, expect);
    }

    #[test]
    fn board_edge_iter_neighbors() {
        let b = Board::new(4, 3);
        let ret: HashSet<_> = b.iter_neighbors(Pos { row: 1, col: 0 }).collect();
        let expect = HashSet::from([
            Pos { row: 0, col: 0 },
            Pos { row: 0, col: 1 },
            Pos { row: 1, col: 1 },
            Pos { row: 2, col: 0 },
            Pos { row: 2, col: 1 },
        ]);
        assert_eq!(ret, expect);
    }

    #[test]
    fn board_center_iter_neighbors() {
        let b = Board::new(4, 3);
        let ret: HashSet<_> = b.iter_neighbors(Pos { row: 1, col: 1 }).collect();
        let expect = HashSet::from([
            Pos { row: 0, col: 0 },
            Pos { row: 0, col: 1 },
            Pos { row: 0, col: 2 },
            Pos { row: 1, col: 0 },
            Pos { row: 1, col: 2 },
            Pos { row: 2, col: 0 },
            Pos { row: 2, col: 1 },
            Pos { row: 2, col: 2 },
        ]);
        assert_eq!(ret, expect);
    }
}
