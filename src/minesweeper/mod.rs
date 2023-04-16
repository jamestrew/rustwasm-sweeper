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

#[derive(Debug, PartialEq, Clone, Copy)]
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
            .filter(|&kind| kind.is_mine())
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
            if self
                .board
                .set(mine_pos, CellKind::Mine { flagged: false })
                .is_ok()
            {
                mines_created += 1;
            }
        }

        self.state = GameState::Playing;
    }

    pub fn flag_cell(&mut self, pos: Pos) {
        if self.state == GameState::Win || self.state == GameState::Lose {
            return;
        }

        match self.board.get(pos) {
            Some(CellKind::Closed { flagged }) => {
                _ = self.board.set(pos, CellKind::Closed { flagged: !flagged })
            }
            Some(CellKind::Mine { flagged }) => {
                _ = self.board.set(pos, CellKind::Mine { flagged: !flagged })
            }
            _ => {}
        }
    }

    pub fn open_cell(&mut self, pos: Pos) {
        match self.state {
            GameState::Unstarted => self.create_mines(Some(pos)),
            GameState::Playing => {}
            _ => return,
        }

        if let Some(kind) = self.board.get(pos) {
            if kind.is_flagged() {
                return;
            }
            match kind {
                CellKind::Mine { flagged: false } => self.state = GameState::Lose,
                CellKind::Closed { .. } => self.check_neighbor(pos),
                _ => (),
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
                    .map_or(false, |&kind| kind.is_mine())
            })
            .count();
        _ = self
            .board
            .set(pos, CellKind::new_open(neighboring_mines as u8));
        self.open_empty_neighbors(pos, neighboring_mines);
        self.check_win_condition();
    }

    fn open_empty_neighbors(&mut self, pos: Pos, neighboring_mines: usize) {
        if neighboring_mines != 0 {
            return;
        }
        self.board
            .iter_neighbors(pos)
            .for_each(|new_pos| self.open_cell(new_pos));
    }

    fn check_win_condition(&mut self) {
        if self.board.iter().flatten().filter(|kind| kind.is_closed()).count() == 0 {
            self.state = GameState::Win;
        }
    }

    pub fn iter_board(&self) -> impl Iterator<Item = Cell> + '_ {
        self.board.iter_pos().filter_map(move |pos| {
            self.board
                .get(pos)
                .map(|cell_kind| Cell::new(pos, *cell_kind, self.state))
        })
    }

    pub fn get(&self, pos: Pos) -> Cell {
        let kind = self
            .board
            .get(pos)
            .unwrap_or(&CellKind::Closed { flagged: false });
        Cell::new(pos, *kind, self.state)
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            for &cell in row.iter() {
                write!(f, "{}", cell.as_char(self.state))?;
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
            .filter(|&kind| kind.is_mine())
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
        assert!(game.board.get(pos).unwrap().is_closed());
        assert_eq!(game.state, GameState::Playing);
    }

    #[test]
    fn flagging_cell_closed() {
        let mut game = Minesweeper::new(4, 3, 3);
        let pos = Pos { row: 0, col: 0 };
        assert!(game.board.get(pos).unwrap().is_closed());
        game.flag_cell(pos);
        assert!(game.board.get(pos).unwrap().is_flagged());
    }

    #[test]
    fn flag_cell_open() {
        let mut game = Minesweeper::new(4, 3, 3);
        let pos = Pos { row: 0, col: 0 };
        game.open_cell(pos);

        assert!(game.board.get(pos).unwrap().is_open());
        game.flag_cell(pos);
        assert!(game.board.get(pos).unwrap().is_open());
    }

    #[test]
    fn flag_cell_flagged() {
        let mut game = Minesweeper::new(4, 3, 3);
        let pos = Pos { row: 0, col: 0 };

        assert!(game.board.get(pos).unwrap().is_closed());
        game.flag_cell(pos);
        assert!(game.board.get(pos).unwrap().is_flagged());
        game.flag_cell(pos);
        assert!(game.board.get(pos).unwrap().is_closed());
    }

    #[test]
    fn flag_cell_mine() {
        let mut game = Minesweeper::new(4, 3, 3);
        let pos = Pos { row: 0, col: 0 };
        _ = game.board.set(pos, CellKind::Mine { flagged: false });

        assert!(game.board.get(pos).unwrap().is_mine());
        game.flag_cell(pos);
        assert!(game.board.get(pos).unwrap().is_flagged());
        game.flag_cell(pos);
        assert!(game.board.get(pos).unwrap().is_mine());
    }

    #[test]
    fn open_cell_with_mine_playing() {
        let mut game = Minesweeper::new(4, 3, 3);
        let pos = Pos { row: 0, col: 0 };
        _ = game.board.set(pos, CellKind::Mine { flagged: false });
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
        assert_eq!(
            game.board.get(pos).unwrap(),
            &CellKind::Open { neighbor_mines: 6 }
        );

        let pos = Pos { row: 0, col: 2 };
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(
            game.board.get(pos).unwrap(),
            &CellKind::Open { neighbor_mines: 2 }
        );

        let pos = Pos { row: 2, col: 1 };
        game.open_cell(pos);
        assert_eq!(game.state, GameState::Win);
        assert_eq!(
            game.board.get(pos).unwrap(),
            &CellKind::Open { neighbor_mines: 4 }
        );
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
        assert!(game.board.get(pos).unwrap().is_open());

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
        assert!(game.board.get(pos).unwrap().is_closed());
        assert_eq!(game.state, GameState::Win);
    }

    #[test]
    fn open_cell_lose() {
        let mut game = Minesweeper::new(9, 9, 10);
        game.state = GameState::Lose;

        let pos = Pos { row: 0, col: 0 };
        game.open_cell(pos);
        assert!(game.board.get(pos).unwrap().is_closed());
        assert_eq!(game.state, GameState::Lose);
    }

    #[test]
    fn open_cell_flagged_mine() {
        let mut game = Minesweeper::new(9, 9, 10);

        let pos = Pos { row: 0, col: 0 };
        game.flag_cell(pos);
        assert!(game.board.get(pos).unwrap().is_flagged());
        game.open_cell(pos);
        assert!(
            game.board.get(pos).unwrap().is_closed(),
            "{:?}",
            game.board.get(pos)
        );
    }

    #[test]
    fn open_cell_to_win() {
        let mut game = Minesweeper::from_matrix(vec![vec![1, 1, 0]]);

        assert_eq!(game.state, GameState::Playing);
        game.open_cell(Pos { row: 0, col: 2 });
        assert_eq!(game.state, GameState::Win);
    }
}
