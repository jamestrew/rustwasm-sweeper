use super::{pos::Pos, GameState};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellKind {
    Open(u8),
    Closed,
    Flagged,
    Mine,
}

#[derive(Debug)]
pub struct Cell {
    pub pos: Pos,
    pub kind: CellKind,
    pub icon: String,
    pub class: String,
}

impl Cell {
    pub fn new(pos: Pos, kind: CellKind, state: &GameState) -> Self {
        let (icon, class) = Self::icon_and_class(kind, state);
        Self {
            pos,
            kind,
            icon: String::from(icon),
            class: String::from(class),
        }
    }

    fn icon_and_class(kind: CellKind, state: &GameState) -> (&'static str, &'static str) {
        match kind {
            CellKind::Closed => ("", "closed"),
            CellKind::Flagged => ("🚩", "flagged"),
            CellKind::Mine => match state {
                GameState::Lose => ("💣", "mine"),
                _ => ("", "closed"),
            },
            CellKind::Open(count) => match count {
                1 => ("1", "open open-1"),
                2 => ("2", "open open-2"),
                3 => ("3", "open open-3"),
                4 => ("4", "open open-4"),
                5 => ("5", "open open-5"),
                6 => ("6", "open open-6"),
                7 => ("7", "open open-7"),
                8 => ("8", "open open-8"),
                _ => ("", "open open-0"),
            },
        }
    }
}
