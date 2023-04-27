use crate::minesweeper::Minesweeper;
use leptos::WriteSignal;

#[derive(Clone, Copy)]
pub struct GameUpdater {
    pub set_game: WriteSignal<Minesweeper>,
}
