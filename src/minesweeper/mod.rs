mod board;
mod cell;

use rand::Rng;
use std::error::Error;
use std::fmt::Display;

pub use board::{Board, Pos};
pub use cell::Cell;

#[derive(Debug, PartialEq)]
pub enum GameState {
    Playing,
    Win,
    Lose,
}

#[derive(Debug)]
pub enum MinesweeperError {
    GameError,
}

impl Error for MinesweeperError {}

impl Display for MinesweeperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MinesweeperError::GameError => f.write_str("Something's fucky"),
        }
    }
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

    pub fn create_mines(&mut self) {
        let mut allowable_mine_pos: Vec<_> = self.board.iter_pos().collect();

        let mut mines_created = 0;
        while mines_created < self.mine_count {
            let index = rand::thread_rng().gen_range(0..allowable_mine_pos.len());
            let mine_pos = allowable_mine_pos.remove(index);
            if self.board.set(mine_pos, Cell::Mine).is_ok() {
                mines_created += 1;
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
        let _ = self.board.set(pos, Cell::Open(neighboring_mines as u8));
        self.open_empty_neighbors(pos, neighboring_mines);
    }

    fn open_empty_neighbors(&mut self, pos: Pos, neighboring_mines: usize) {
        if neighboring_mines != 0 {
            return;
        }
        self.board
            .iter_neighbors(pos)
            .for_each(|new_pos| self.open_cell(new_pos));
    }
}

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

    #[test]
    fn open_cell_with_neighboring_mines() {
        let board = vec![vec![1, 1, 0], vec![1, 0, 1], vec![1, 0, 1]];
        let mut game = Minesweeper::from_matrix(board);

        let pos = Pos { row: 1, col: 1 };
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.board.get(pos).unwrap(), &Cell::Open(6));

        let pos = Pos { row: 0, col: 2 };
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.board.get(pos).unwrap(), &Cell::Open(2));

        let pos = Pos { row: 2, col: 1 };
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.board.get(pos).unwrap(), &Cell::Open(4));
    }

    #[test]
    fn open_cell_chaining_neighbors() {
        let board = vec![vec![1, 1, 0, 0], vec![1, 0, 0, 0], vec![1, 0, 0, 0]];
        let mut game = Minesweeper::from_matrix(board);

        let pos = Pos { row: 2, col: 3 };
        game.open_cell(pos);

        let expect = "\
        xx10\n\
        x410\n\
        x200\n";
        let display = format!("{}", game);
        assert_eq!(expect, display);
    }
}
