use super::pos::Pos;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellKind {
    Open(u8),
    Closed,
    Flagged,
    Mine,
}

pub struct CellHighlights {
    pub fg: String,
    pub bg: String,
}

impl CellHighlights {
    const OPEN_BG: &str = "#c0c0c0";

    pub fn new(fg: &str, bg: &str) -> Self {
        let bg = if bg == "" { Self::OPEN_BG } else { bg };
        let fg = if fg == "" { "black" } else { fg };
        Self {
            fg: String::from(fg),
            bg: String::from(bg),
        }
    }
}

pub struct Cell {
    pub pos: Pos,
    pub kind: CellKind,
    pub icon: String,
    pub hl: CellHighlights,
    pub class: String,
}

impl Cell {
    pub fn new(pos: Pos, kind: CellKind) -> Self {
        let (icon, hl, class) = Self::icon_and_highlight(kind);
        Self {
            pos,
            kind,
            icon: String::from(icon),
            hl,
            class: String::from(class)
        }
    }

    fn icon_and_highlight(kind: CellKind) -> (&'static str, CellHighlights, &'static str) {
        match kind {
            CellKind::Closed => ("", CellHighlights::new("", "white"), "closed"),
            CellKind::Flagged => ("🚩", CellHighlights::new("", "white"), "flagged"),
            CellKind::Mine => ("💣", CellHighlights::new("", "#c0c0c0"), "mine"),
            CellKind::Open(count) => match count {
                1 => ("1", CellHighlights::new("#0000ff", ""), "open-1"),
                2 => ("2", CellHighlights::new("#008200", ""), "open-2"),
                3 => ("3", CellHighlights::new("#ff0000", ""), "open-3"),
                4 => ("4", CellHighlights::new("#000084", ""), "open-4"),
                5 => ("5", CellHighlights::new("#840000", ""), "open-5"),
                6 => ("6", CellHighlights::new("#008284", ""), "open-6"),
                7 => ("7", CellHighlights::new("#840084", ""), "open-7"),
                8 => ("8", CellHighlights::new("#757575", ""), "open-8"),
                _ => ("", CellHighlights::new("", ""), "open-0"),
            },
        }
    }
}
