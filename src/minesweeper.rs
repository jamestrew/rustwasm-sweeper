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

#[derive(Eq, PartialEq, Hash)]
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

    pub fn action_cell(&mut self, pos: Pos) {
        match self.board[pos.row][pos.col] {
            Cell::Mine => self.state = GameState::Lose,
            Cell::Closed => self.open_cell(pos),
            _ => return,
        }
    }

    fn open_cell(&mut self, pos: Pos) {
        self.iter_neighbors(pos).for_each(|_| todo!())
    }

    fn iter_neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> {
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

        (row_start..=row_end).flat_map(move |r| {
            (col_start..=col_end).filter_map(move |c| {
                if r == pos.row && c == pos.col {
                    None
                } else {
                    Some(Pos { row: r, col: c })
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_board_init() {
        let expect = vec![
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
            vec![Cell::Closed, Cell::Closed, Cell::Closed],
        ];
        let result = Minesweeper::new(4, 3, 0);
        assert_eq!(expect.len(), result.board.len());
        for i in 0..expect.len() {
            assert_eq!(expect[i].len(), result.board[i].len());
            for j in 0..expect[i].len() {
                assert_eq!(expect[i][j], result.board[i][j]);
            }
        }
        assert_eq!(result.mine_count, 0);
        assert_eq!(result.height, 4);
        assert_eq!(result.width, 3);
        assert_eq!(result.state, GameState::Playing);
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
}
