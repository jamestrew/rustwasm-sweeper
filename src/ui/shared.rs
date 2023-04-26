use crate::minesweeper::{Minesweeper, Pos};
use leptos::WriteSignal;

#[derive(Clone, Copy)]
pub struct GameUpdater {
    pub set_game: WriteSignal<Minesweeper>,
    pub set_chord_pos: WriteSignal<Option<Pos>>,
}
