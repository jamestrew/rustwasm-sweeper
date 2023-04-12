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
}

impl Cell {
    pub fn new(pos: Pos, kind: CellKind) -> Self {
        let (icon, hl) = Self::icon_and_highlight(kind);
        Self {
            pos,
            kind,
            icon: String::from(icon),
            hl,
        }
    }

    fn icon_and_highlight(kind: CellKind) -> (&'static str, CellHighlights) {
        match kind {
            CellKind::Closed => ("", CellHighlights::new("", "white")),
            CellKind::Flagged => ("🚩", CellHighlights::new("", "white")),
            CellKind::Mine => ("💣", CellHighlights::new("", "#c0c0c0")),
            CellKind::Open(count) => match count {
                1 => ("1", CellHighlights::new("#0000ff", "")),
                2 => ("2", CellHighlights::new("#008200", "")),
                3 => ("3", CellHighlights::new("#ff0000", "")),
                4 => ("4", CellHighlights::new("#000084", "")),
                5 => ("5", CellHighlights::new("#840000", "")),
                6 => ("6", CellHighlights::new("#008284", "")),
                7 => ("7", CellHighlights::new("#840084", "")),
                8 => ("8", CellHighlights::new("#757575", "")),
                _ => ("", CellHighlights::new("", "")),
            },
        }
    }
}

/*
const cellIcon = new Map<number, Icon>([
  [-2, { icon: "🚩", background: "white", color: "" }],
  [-1, { icon: "", background: "white", color: "" }],
  [0, { icon: "", background: OPEN_COLOR, color: "" }],
  [1, { icon: "1", background: OPEN_COLOR, color: "#0000ff" }],
  [2, { icon: "2", background: OPEN_COLOR, color: "#008200" }],
  [3, { icon: "3", background: OPEN_COLOR, color: "#ff0000" }],
  [4, { icon: "4", background: OPEN_COLOR, color: "#000084" }],
  [5, { icon: "5", background: OPEN_COLOR, color: "#840000" }],
  [6, { icon: "6", background: OPEN_COLOR, color: "#008284" }],
  [7, { icon: "7", background: OPEN_COLOR, color: "#840084" }],
  [8, { icon: "8", background: OPEN_COLOR, color: "#757575" }],
  [9, { icon: "💣", background: OPEN_COLOR, color: "" }],
]);

*/
