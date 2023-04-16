use super::{pos::Pos, GameState};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellKind {
    Open { neighbor_mines: u8 },
    Closed { flagged: bool },
    Mine { flagged: bool },
}

impl CellKind {
    pub fn new_closed() -> Self {
        CellKind::Closed {flagged: false}
    }

    pub fn new_mine() -> Self {
        CellKind::Mine {flagged: false}
    }

    pub fn is_flagged(&self) -> bool {
        match self {
            CellKind::Open { .. } => false,
            CellKind::Closed { flagged } => *flagged,
            CellKind::Mine { flagged } => *flagged,
        }
    }

    pub fn is_mine(&self) -> bool {
        match self {
            Self::Mine { .. } => true,
            _ => false,
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::Open { .. } => true,
            _ => false,
        }
    }

    // TODO: maybe we can have some macro fun with these is_* methods
    pub fn is_closed(&self) -> bool {
        match self {
            Self::Closed { .. } => true,
            _ => false,
        }
    }

    pub fn as_char(&self, state: GameState) -> char {
        if self.is_flagged() {
            return '!';
        }
        match self {
            CellKind::Open { neighbor_mines } => (b'0' + neighbor_mines) as char,
            CellKind::Closed { .. } => 'x',
            CellKind::Mine { .. } => match state {
                GameState::Lose => '!',
                _ => 'x',
            },
        }
    }

    pub fn as_icon_and_hl(&self, state: GameState) -> (&'static str, &'static str) {
        if self.is_flagged() {
            return ("🚩", "flagged");
        }
        match self {
            CellKind::Closed { .. } => ("", "closed"),
            CellKind::Mine { .. } => match state {
                GameState::Lose => ("💣", "mine"),
                _ => ("", "closed"),
            },
            CellKind::Open {
                neighbor_mines: count,
                ..
            } => match count {
                1 => ("1", "open open-1"),
                2 => ("2", "open open-2"),
                3 => ("3", "open open-3"),
                4 => ("4", "open open-4"),
                5 => ("5", "open open-5"),
                6 => ("6", "open open-6"),
                7 => ("7", "open open-7"),
                8 => ("8", "open open-8"),
                _ => ("", "open"),
            },
        }
    }
}

#[derive(Debug)]
pub struct Cell {
    pub pos: Pos,
    pub kind: CellKind,
    pub icon: String,
    pub class: String,
}

impl Cell {
    pub fn new(pos: Pos, kind: CellKind, state: GameState) -> Self {
        let (icon, class) = kind.as_icon_and_hl(state);
        Self {
            pos,
            kind,
            icon: String::from(icon),
            class: String::from(class),
        }
    }
}
