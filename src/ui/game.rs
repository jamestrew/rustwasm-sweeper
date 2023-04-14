use crate::minesweeper::{Cell, Minesweeper, Pos};
use leptos::*;
use leptos_meta::{Title, TitleProps};

const CELL_SIZE: usize = 30;

#[derive(Clone, Copy)]
struct GameUpdater {
    set_game: WriteSignal<Minesweeper>,
}

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (game, set_game) = create_signal(cx, Minesweeper::new(9, 9, 10));
    provide_context(cx, GameUpdater { set_game });

    let board_cells = move || {
        game.with(|g| {
            g.iter_board()
                .map(|cell| create_rw_signal(cx, cell))
                .collect::<Vec<RwSignal<Cell>>>()
        })
    };
    let style = move || {
        game.with(|g| {
            format!(
                "height: {}px; width: {}px",
                g.board.height as usize * CELL_SIZE,
                g.board.width as usize * CELL_SIZE
            )
        })
    };

    let open = move |_| {
        set_game.update(|game| game.open_cell(Pos { row: 0, col: 0 }));
        log!("opening");
    };

    create_effect(cx, move |_| {
        game.with(|game_state| {
            log!("Game:\n{}", game_state);
        });
    });

    view! { cx,
        <Title text="Minesweeper" />

        <div class="Board" style=style()>
            <For
                each=move || board_cells()
                key=|cell| cell.with(|c| c.pos)
                view=move |cx, cell| {
                        view! { cx,
                            <CellComp cell />
                        }
                    }
            />
        </div>
        <button on:click=open>"Open"</button>
    }
}

#[component]
pub fn CellComp(cx: Scope, cell: RwSignal<Cell>) -> impl IntoView {
    let GameUpdater { set_game } = use_context(cx).unwrap();

    let click = move |_| {
        set_game.update(|game| {
            game.open_cell(Pos {
                row: cell.with(|c| c.pos.row),
                col: cell.with(|c| c.pos.col),
            })
        });
        log!("opening");
    };

    create_effect(cx, move |_| {
        cell.with(|c| {
            log!("cell: ({},{}) {}", c.pos.row, c.pos.col, c.icon);
        });
    });

    let class = format!("Cell {}", cell.with(|c| c.class.clone()));
    let style = format!(
        "grid-column-start: {}; grid-row-start: {};",
        cell.with(|c| c.pos.col + 1),
        cell.with(|c| c.pos.row + 1)
    );

    view! { cx,
        <div
            class=class
            style=style
            on:click=click
        >
            {move || cell.with(|c| c.icon.clone())}
        </div>
    }
}
