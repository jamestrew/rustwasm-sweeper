use crate::minesweeper::{Cell, Minesweeper, Pos};
use leptos::*;
use leptos_meta::{Title, TitleProps};

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (game, set_game) = create_signal(cx, Minesweeper::new(9, 9, 10));

    let open = move |_| {
        set_game.update(|game| game.open_cell(Pos { row: 0, col: 0 }));
        log!("opening");
    };

    create_effect(cx, move |_| game.with(|gs| log!("Game: {}", gs)));

    view! { cx,
        <Title text="Minesweeper" />
        <p> { move || game.with(|gs| gs.to_string())} </p>
        <button on:click=open>"Open"</button>
    }
}

#[component]
pub fn Cell(cx: Scope, _game: Minesweeper, cell: Cell) -> impl IntoView {
    const CELL_SIZE: usize = 30;
    let style = format!(
        r#"
            width: {}px,
            height: {}px`,
            gridColumnStart: {},
            gridRowStart: {},
            background: {},
            fontWeight: "525",
            color: {},
        "#,
        CELL_SIZE - 6,
        CELL_SIZE - 6,
        cell.pos.col + 1,
        cell.pos.row + 1,
        cell.hl.bg,
        cell.hl.fg
    );

    view! {
        cx,
        <div
            class="Cell"
            style=style
        >
            {cell.icon}
        </div>
    }
}

/*

    <div
      className="Cell"
      style={{
        width: `${CELL_SIZE - 6}px`,
        height: `${CELL_SIZE - 6}px`,
        gridColumnStart: x + 1,
        gridRowStart: y + 1,
        background: highlight && cellType !== -2 ? OPEN_COLOR : background,
        fontWeight: "525",
        color: color,
      }}
      onContextMenu={(e) => e.preventDefault()}
      onMouseDown={(e) => handleMouseDown(e.buttons)}
      onMouseUp={() => sendAction({ x, y })}
    >
      {icon}
    </div>
*/
