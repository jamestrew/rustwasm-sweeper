mod board;
mod cell;
mod pos;

use rand::Rng;
use std::error::Error;
use std::fmt::Display;

use board::Board;
pub use cell::Cell;
pub use cell::CellKind;
pub use pos::Pos;

#[derive(Debug, PartialEq)]
pub enum GameState {
    Unstarted,
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
            state: GameState::Unstarted,
        }
    }

    pub fn from_matrix(matrix: Vec<Vec<i32>>) -> Self {
        let board = Board::from_matrix(matrix);
        let mine_count = board
            .iter()
            .flatten()
            .filter(|&kind| *kind == CellKind::Mine)
            .count();
        Self {
            mine_count,
            board,
            state: GameState::Playing,
        }
    }

    pub fn create_mines(&mut self, start_pos: Option<Pos>) {
        let mut allowable_mine_pos: Vec<_> = self
            .board
            .iter_pos()
            .filter(|&pos| Some(pos) != start_pos)
            .collect();

        let mut mines_created = 0;
        while mines_created < self.mine_count {
            let index = rand::thread_rng().gen_range(0..allowable_mine_pos.len());
            let mine_pos = allowable_mine_pos.remove(index);
            if self.board.set(mine_pos, CellKind::Mine).is_ok() {
                mines_created += 1;
            }
        }

        self.state = GameState::Playing;
    }

    pub fn flag_cell(&mut self, pos: Pos) {
        if let Some(CellKind::Closed) = self.board.get(pos) {
            _ = self.board.set(pos, CellKind::Flagged);
        }
    }

    pub fn open_cell(&mut self, pos: Pos) {
        match self.state {
            GameState::Unstarted => self.create_mines(Some(pos)),
            GameState::Playing => {}
            _ => return,
        }

        if let Some(kind) = self.board.get(pos) {
            match kind {
                CellKind::Mine => self.state = GameState::Lose,
                CellKind::Closed => self.check_neighbor(pos),
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
                    .map_or(false, |&kind| kind == CellKind::Mine)
            })
            .count();
        _ = self.board.set(pos, CellKind::Open(neighboring_mines as u8));
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

    pub fn iter_board(&self) -> impl Iterator<Item = Cell> + '_ {
        self.board.iter_pos().filter_map(move |pos| {
            self.board
                .get(pos)
                .map(|cell_kind| Cell::new(pos, *cell_kind))
        })
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            for &cell in row.iter() {
                let ch = match cell {
                    CellKind::Open(count) => (b'0' + count) as char,
                    CellKind::Closed => 'x',
                    CellKind::Flagged => 'F',
                    CellKind::Mine => match self.state {
                        GameState::Unstarted => 'x',
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

    fn count_mines(game: &Minesweeper) -> usize {
        game.board
            .iter()
            .flatten()
            .filter(|&kind| *kind == CellKind::Mine)
            .count()
    }

    #[test]
    fn basic_board_init() {
        let result = Minesweeper::new(4, 3, 0);
        assert_eq!(result.mine_count, 0);
        assert_eq!(result.state, GameState::Unstarted);
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
    fn populate_board_with_some_mines_freely() {
        let mut game = Minesweeper::new(4, 3, 3);
        assert_eq!(game.state, GameState::Unstarted);
        game.create_mines(None);
        assert_eq!(count_mines(&game), game.mine_count);
        assert_eq!(game.state, GameState::Playing);
    }

    #[test]
    fn populate_board_with_some_mines_after_first_click() {
        let mut game = Minesweeper::new(4, 3, 11);
        assert_eq!(game.state, GameState::Unstarted);
        let pos = Pos { row: 0, col: 0 };
        game.create_mines(Some(pos));
        assert_eq!(count_mines(&game), game.mine_count);
        assert_eq!(game.board.get(pos).unwrap(), &CellKind::Closed);
        assert_eq!(game.state, GameState::Playing);
    }

    #[test]
    fn flagging_cell() {
        let mut game = Minesweeper::new(4, 3, 3);
        let pos = Pos { row: 0, col: 0 };
        game.flag_cell(pos);
        assert_eq!(*game.board.get(pos).unwrap(), CellKind::Flagged);
    }

    #[test]
    fn open_cell_with_mine_playing() {
        let mut game = Minesweeper::new(4, 3, 3);
        let pos = Pos { row: 0, col: 0 };
        _ = game.board.set(pos, CellKind::Mine);
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Lose);
    }

    #[test]
    fn open_cell_with_neighboring_mines_playing() {
        let board = vec![vec![1, 1, 0], vec![1, 0, 1], vec![1, 0, 1]];
        let mut game = Minesweeper::from_matrix(board);

        let pos = Pos { row: 1, col: 1 };
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.board.get(pos).unwrap(), &CellKind::Open(6));

        let pos = Pos { row: 0, col: 2 };
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.board.get(pos).unwrap(), &CellKind::Open(2));

        let pos = Pos { row: 2, col: 1 };
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.board.get(pos).unwrap(), &CellKind::Open(4));
    }

    #[test]
    fn open_cell_chaining_neighbors_playing() {
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

    #[test]
    fn open_cell_unstarted() {
        let mut game = Minesweeper::new(9, 9, 10);
        assert_eq!(game.state, GameState::Unstarted);
        assert_eq!(game.mine_count, 10);
        assert_eq!(count_mines(&game), 0);

        let pos = Pos { row: 0, col: 0 };
        game.open_cell(pos);
        let &cell_kind = game.board.get(pos).unwrap();
        match cell_kind {
            CellKind::Open(_) => assert!(true),
            _ => assert!(false, "Expected Open CellKind variant"),
        }

        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.mine_count, 10);
        assert_eq!(count_mines(&game), 10);
    }

    #[test]
    fn open_cell_win() {
        let mut game = Minesweeper::new(9, 9, 10);
        game.state = GameState::Win;

        let pos = Pos { row: 0, col: 0 };
        game.open_cell(pos);
        assert_eq!(game.board.get(pos).unwrap(), &CellKind::Closed);
        assert_eq!(game.state, GameState::Win);
    }

    #[test]
    fn open_cell_lose() {
        let mut game = Minesweeper::new(9, 9, 10);
        game.state = GameState::Lose;

        let pos = Pos { row: 0, col: 0 };
        game.open_cell(pos);
        assert_eq!(game.board.get(pos).unwrap(), &CellKind::Closed);
        assert_eq!(game.state, GameState::Lose);
    }
}
