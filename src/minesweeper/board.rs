use super::cell::{Cell, CellKind};
use super::pos::Pos;
use std::error::Error;
use std::fmt::Display;
use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub enum BoardError {
    SetCellError { pos: Pos, kind: CellKind },
}

impl Error for BoardError {}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::SetCellError { pos, kind } => {
                write!(f, "failed to set {:?} at {:?}", kind, pos)
            }
        }
    }
}

pub struct Board {
    b: Vec<Vec<CellKind>>,
    pub height: u8,
    pub width: u8,
}

impl Board {
    pub fn new(height: u8, width: u8) -> Self {
        let b = (0..height)
            .map(|_| (0..width).map(|_| CellKind::Closed).collect())
            .collect();

        Self { b, height, width }
    }

    pub fn from_matrix(matrix: Vec<Vec<i32>>) -> Self {
        let height = matrix.len() as u8;
        let width = matrix.get(0).map_or(0, |row| row.len()) as u8;
        let b = matrix
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|val| {
                        if val == 0 {
                            CellKind::Closed
                        } else {
                            CellKind::Mine
                        }
                    })
                    .collect()
            })
            .collect();
        Self { b, height, width }
    }

    pub fn get(&self, pos: Pos) -> Option<&CellKind> {
        self.b
            .get(pos.row as usize)
            .and_then(|row| row.get(pos.col as usize))
    }

    pub fn set(&mut self, pos: Pos, kind: CellKind) -> Result<(), BoardError> {
        self.b
            .get_mut(pos.row as usize)
            .ok_or_else(|| BoardError::SetCellError { pos, kind })
            .and_then(|row| {
                row.get_mut(pos.col as usize)
                    .ok_or_else(|| BoardError::SetCellError { pos, kind })
            })
            .map(|c| *c = kind)
    }

    pub fn iter(&self) -> Iter<Vec<CellKind>> {
        self.b.iter()
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        self.iter_pos()
            .filter_map(move |pos| self.get(pos).map(|cell_kind| Cell::new(pos, *cell_kind)))
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> {
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |row| (0..width).map(move |col| Pos { row, col }))
    }

    pub fn iter_neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        let last_row = self.height as i8 - 1;
        let row_start = (pos.row as i8 - 1).clamp(0, last_row) as u8;
        let row_end = (pos.row as i8 + 1).clamp(0, last_row) as u8;

        let last_col = self.width as i8 - 1;
        let col_start = (pos.col as i8 - 1).clamp(0, last_col) as u8;
        let col_end = (pos.col as i8 + 1).clamp(0, last_col) as u8;

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

    fn assert_board(actual: Board, expect: Vec<Vec<CellKind>>) {
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
            vec![CellKind::Closed, CellKind::Closed, CellKind::Closed],
            vec![CellKind::Closed, CellKind::Closed, CellKind::Closed],
            vec![CellKind::Closed, CellKind::Closed, CellKind::Closed],
            vec![CellKind::Closed, CellKind::Closed, CellKind::Closed],
        ];
        assert_board(b, expect);
    }

    #[test]
    fn init_from_matrix() {
        let board = vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0]];
        let expect = vec![
            vec![
                CellKind::Closed,
                CellKind::Mine,
                CellKind::Closed,
                CellKind::Mine,
            ],
            vec![
                CellKind::Mine,
                CellKind::Closed,
                CellKind::Mine,
                CellKind::Closed,
            ],
        ];
        let b = Board::from_matrix(board);
        assert_board(b, expect);
    }

    #[test]
    fn board_get_inbounds() {
        let mut b = Board::new(4, 3);
        b.b[0][1] = CellKind::Mine;
        assert_eq!(b.get(Pos { row: 0, col: 1 }).unwrap(), &CellKind::Mine);
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
        assert_eq!(b.get(pos).unwrap(), &CellKind::Closed);
        assert_eq!(b.set(pos, CellKind::Mine).unwrap(), ());
        assert_eq!(b.get(pos).unwrap(), &CellKind::Mine);
    }

    #[test]
    fn board_set_outbounds() {
        let mut b = Board::new(4, 3);
        let pos = Pos { row: 100, col: 0 };
        let res = b.set(pos, CellKind::Mine);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            BoardError::SetCellError {
                pos,
                kind: CellKind::Mine
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
