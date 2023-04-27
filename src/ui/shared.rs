use crate::minesweeper::Minesweeper;
use leptos::WriteSignal;

#[derive(Clone, Copy)]
pub struct GameUpdater {
    pub set_game: WriteSignal<Minesweeper>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MouseButtons {
    None,
    LClick,
    RClick,
    LRClick,
    Others(u16),
}

impl MouseButtons {
    pub fn from_buttons(buttons: u16) -> Self {
        match buttons {
            0 => Self::None,
            1 => Self::LClick,
            2 => Self::RClick,
            3 => Self::LRClick,
            x => Self::Others(x),
        }
    }
}

pub const CELL_SIZE: usize = 30;
