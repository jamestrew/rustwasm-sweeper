use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Cell {
    Open(usize),
    Closed,
    Flagged,
    Mine,
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Playing,
    Win,
    Lose,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

pub struct Minesweeper {
    pub height: usize,
    pub width: usize,
    pub mine_count: usize,
    pub board: Vec<Vec<Cell>>,
    pub state: GameState,
}

impl Minesweeper {
    pub fn new(height: usize, width: usize, mine_count: usize) -> Self {
        let board = (0..height)
            .map(|_| (0..width).map(|_| Cell::Closed).collect())
            .collect();

        Minesweeper {
            height,
            width,
            mine_count,
            board,
            state: GameState::Playing,
        }
    }

    pub fn from_vec(vec: Vec<Vec<usize>>) -> Self {
        let mut game = Self::new(vec.len(), vec[0].len(), 0);
        for i in 0..game.height {
            for j in 0..game.width {
                if vec[i][j] != 0 {
                    game.board[i][j] = Cell::Mine;
                }
            }
        }
        game
    }

    pub fn rand_mine_pos(&self) -> Pos {
        let mut tr = rand::thread_rng();
        Pos {
            row: tr.gen_range(0..self.height),
            col: tr.gen_range(0..self.width),
        }
    }

    pub fn create_mines(&mut self) {
        let mut mines_created: usize = 0;
        let mut mine;
        while mines_created < self.mine_count {
            mine = self.rand_mine_pos();
            if self.board[mine.row][mine.col] == Cell::Closed {
                self.board[mine.row][mine.col] = Cell::Mine;
                mines_created += 1;
            }
        }
    }

    pub fn flag_cell(&mut self, pos: Pos) {
        match self.board[pos.row][pos.col] {
            Cell::Closed => self.board[pos.row][pos.col] = Cell::Flagged,
            _ => return,
        }
    }

    pub fn lclick_cell(&mut self, pos: Pos) {
        match self.board[pos.row][pos.col] {
            Cell::Mine => self.state = GameState::Lose,
            Cell::Closed => self.open_cell(pos),
            _ => return,
        }
    }

    fn open_cell(&mut self, pos: Pos) {
        let neighboring_mines = self
            .iter_neighbors(&pos)
            .filter(|cell| self.board[cell.row][cell.col] == Cell::Mine)
            .count();
        self.board[pos.row][pos.col] = Cell::Open(neighboring_mines);
        self.open_empty_neighbors(&pos);
    }

    fn open_empty_neighbors(&mut self, _pos: &Pos) {
        // self.iter_neighbors(&pos)
        //     .filter(|cell| self.board[cell.row][cell.col] == Cell::Closed);
    }

    fn iter_neighbors(&self, pos: &Pos) -> impl Iterator<Item = Pos> {
        let row_start = if pos.row > 0 { pos.row - 1 } else { pos.row };
        let row_end = if pos.row < self.height - 1 {
            pos.row + 1
        } else {
            pos.row
        };
        let col_start = if pos.col > 0 { pos.col - 1 } else { pos.col };
        let col_end = if pos.col < self.width - 1 {
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

    fn assert_board(actual: Vec<Vec<Cell>>, expect: Vec<Vec<Cell>>) {
        assert_eq!(expect.len(), actual.len());
        for i in 0..expect.len() {
            assert_eq!(expect[i].len(), actual[i].len());
            for j in 0..expect[i].len() {
                assert_eq!(expect[i][j], actual[i][j]);
            }
        }
    }

    #[test]
    fn basic_board_init() {
        let expect = vec![
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
        ];
        let result = Minesweeper::new(4, 3, 0);
        assert_board(result.board, expect);
        assert_eq!(result.mine_count, 0);
        assert_eq!(result.height, 4);
        assert_eq!(result.width, 3);
        assert_eq!(result.state, GameState::Playing);
    }

    #[test]
    fn board_init_from_vec() {
        let board = vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0]];
        let expect = vec![
            vec![Cell::Closed, Cell::Mine, Cell::Closed, Cell::Mine],
            vec![Cell::Mine, Cell::Closed, Cell::Mine, Cell::Closed],
        ];
        let game = Minesweeper::from_vec(board);
        assert_board(game.board, expect);
    }

    #[test]
    fn populate_board_with_some_mines() {
        let mut game = Minesweeper::new(4, 3, 3);
        game.create_mines();
        let mine_count = game
            .board
            .iter()
            .flatten()
            .filter(|&cell| *cell == Cell::Mine)
            .count();
        assert_eq!(mine_count, game.mine_count);
    }

    #[test]
    fn flagging_cell() {
        let mut game = Minesweeper::new(4, 3, 3);
        game.flag_cell(Pos { row: 0, col: 0 });
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.board[0][0], Cell::Flagged);
    }

    #[test]
    fn lclick_cell_with_mine() {
        let mut game = Minesweeper::new(4, 3, 3);
        game.board[0][0] = Cell::Mine;
        assert_eq!(game.state, GameState::Playing);
        game.lclick_cell(Pos { row: 0, col: 0 });
        assert_eq!(game.state, GameState::Lose);
    }

    #[test]
    fn lclick_cell_surrounded_by_mines() {
        let mut game = Minesweeper::from_vec(vec![vec![0, 1], vec![1, 1]]);
        assert_eq!(game.state, GameState::Playing);
        game.lclick_cell(Pos { row: 0, col: 0 });
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.board[0][0], Cell::Open(3));
    }

    #[test]
    fn board_corner_iter_neighbors() {
        let game = Minesweeper::new(4, 3, 3);
        let ret: HashSet<_> = game.iter_neighbors(&Pos { row: 0, col: 0 }).collect();
        let expect = HashSet::from([
            Pos { row: 0, col: 1 },
            Pos { row: 1, col: 0 },
            Pos { row: 1, col: 1 },
        ]);
        assert_eq!(ret, expect);
    }

    #[test]
    fn board_edge_iter_neighbors() {
        let game = Minesweeper::new(4, 3, 3);
        let ret: HashSet<_> = game.iter_neighbors(&Pos { row: 1, col: 0 }).collect();
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
        let game = Minesweeper::new(4, 3, 3);
        let ret: HashSet<_> = game.iter_neighbors(&Pos { row: 1, col: 1 }).collect();
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
