mod board;
mod cell;

use rand::Rng;
use std::fmt::Display;

pub use board::{Board, Pos};
pub use cell::Cell;

#[derive(Debug, PartialEq)]
pub enum GameState {
    Playing,
    Win,
    Lose,
}

pub struct Minesweeper {
    pub mine_count: usize,
    pub board: Board,
    pub state: GameState,
}

impl Minesweeper {
    pub fn new(height: u8, width: u8, mine_count: usize) -> Self {
        Minesweeper {
            mine_count,
            board: Board::new(height, width),
            state: GameState::Playing,
        }
    }

    pub fn from_matrix(matrix: Vec<Vec<i32>>) -> Self {
        let board = Board::from_matrix(matrix);
        let mine_count = board
            .iter()
            .flatten()
            .filter(|&cell| *cell == Cell::Mine)
            .count();
        Self {
            mine_count,
            board,
            state: GameState::Playing,
        }
    }

    pub fn rand_mine_pos(&self) -> Pos {
        let mut tr = rand::thread_rng();
        Pos {
            row: tr.gen_range(0..self.board.height),
            col: tr.gen_range(0..self.board.width),
        }
    }

    pub fn create_mines(&mut self) {
        let mut mines_created: usize = 0;
        let mut mine_pos;
        while mines_created < self.mine_count {
            mine_pos = self.rand_mine_pos();
            if let Some(Cell::Closed) = self.board.get(mine_pos) {
                if self.board.set(mine_pos, Cell::Mine).is_ok() {
                    mines_created += 1;
                }
            }
        }
    }

    pub fn flag_cell(&mut self, pos: Pos) {
        if let Some(Cell::Closed) = self.board.get(pos) {
            let _ = self.board.set(pos, Cell::Flagged);
        }
    }

    pub fn open_cell(&mut self, pos: Pos) {
        if let Some(cell) = self.board.get(pos) {
            match cell {
                Cell::Mine => self.state = GameState::Lose,
                Cell::Closed => self.check_neighbor(pos),
                _ => return,
            }
        }
    }

    fn check_neighbor(&mut self, pos: Pos) {
        let neighboring_mines = self
            .board
            .iter_neighbors(pos)
            .filter(|&cell_pos| {
                self.board
                    .get(cell_pos)
                    .map_or(false, |&cell| cell == Cell::Mine)
            })
            .count();
        let _ = self.board.set(pos, Cell::Open(neighboring_mines));
        self.open_empty_neighbors(pos);
    }

    fn open_empty_neighbors(&mut self, _pos: Pos) {
        // self.iter_neighbors(&pos)
        //     .filter(|cell| self.board[cell.row][cell.col] == Cell::Closed);
impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            for &cell in row.iter() {
                let ch = match cell {
                    Cell::Open(count) => (b'0' + count) as char,
                    Cell::Closed => 'x',
                    Cell::Flagged => 'F',
                    Cell::Mine => match self.state {
                        GameState::Playing => 'x',
                        GameState::Lose => '!',
                        GameState::Win => return Err(std::fmt::Error),
                    },
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_board_init() {
        let result = Minesweeper::new(4, 3, 0);
        assert_eq!(result.mine_count, 0);
        assert_eq!(result.state, GameState::Playing);
    }

    #[test]
    fn board_init_from_vec() {
        let board = vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0]];
        let game = Minesweeper::from_matrix(board);
        assert_eq!(game.mine_count, 4);
        assert_eq!(game.state, GameState::Playing);
    }

    #[test]
    fn print_game_initial() {
        let board = vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0]];
        let game = Minesweeper::from_matrix(board);

        let display = format!("{}", game);
        let expect = "xxxx\nxxxx\n";
        assert_eq!(expect, display);
    }

    #[test]
    fn print_game_playing() {
        let board = vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0]];
        let mut game = Minesweeper::from_matrix(board);
        game.open_cell(Pos { row: 0, col: 0 });
        game.open_cell(Pos { row: 1, col: 3 });

        let display = format!("{}", game);
        let expect = "2xxx\nxxx2\n";
        assert_eq!(expect, display);
    }

    #[test]
    #[should_panic]
    fn print_game_won() {
        let board = vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0]];
        let mut game = Minesweeper::from_matrix(board);
        game.open_cell(Pos { row: 0, col: 0 });
        game.open_cell(Pos { row: 1, col: 3 });
        game.state = GameState::Win;

        game.to_string();
    }

    #[test]
    fn print_game_lose() {
        let board = vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0]];
        let mut game = Minesweeper::from_matrix(board);
        game.open_cell(Pos { row: 0, col: 0 });
        game.open_cell(Pos { row: 0, col: 1 });

        let display = format!("{}", game);
        let expect = "2!x!\n!x!x\n";
        assert_eq!(expect, display);
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
        let pos = Pos { row: 0, col: 0 };
        game.flag_cell(pos);
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(*game.board.get(pos).unwrap(), Cell::Flagged);
    }

    #[test]
    fn open_cell_with_mine() {
        let mut game = Minesweeper::new(4, 3, 3);
        let pos = Pos { row: 0, col: 0 };
        let _ = game.board.set(pos, Cell::Mine);
        assert_eq!(game.state, GameState::Playing);
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Lose);
    }
}
