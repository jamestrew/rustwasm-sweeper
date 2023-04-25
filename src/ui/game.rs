use crate::minesweeper::{Minesweeper, Pos};
use crate::ui::shared::GameUpdater;
use leptos::{ev::MouseEvent, *};
use leptos_meta::{Title, TitleProps};

use crate::ui::settings::*;

const CELL_SIZE: usize = 30;

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (game, set_game) = create_signal(cx, Minesweeper::new(9, 9, 10));
    let board_pos = move || game.with(|g| g.board.iter_pos().collect::<Vec<Pos>>());
    provide_context(cx, GameUpdater { set_game });

    let style = move || {
        game.with(|g| {
            format!(
                "height: {}px; width: {}px",
                g.board.height as usize * CELL_SIZE,
                g.board.width as usize * CELL_SIZE
            )
        })
    };

    create_effect(cx, move |_| {
        game.with(|game_state| {
            log!("Game:\n{}", game_state);
        });
    });

    view! { cx,
        <Title text="Minesweeper" />

        <div class="game">
            <div class="Board" style=style>
                <For
                    each=board_pos
                    key=|&pos| pos
                    view=move |cx, pos| {
                        view! { cx,
                            <Cell game pos />
                        }
                    }
                />
            </div>
            <SettingsPanel />
        </div>
    }
}

#[component]
pub fn Cell(cx: Scope, game: ReadSignal<Minesweeper>, pos: Pos) -> impl IntoView {
    let (mouse_down, set_mouse_down) = create_signal::<Option<i16>>(cx, None);
    let GameUpdater { set_game } = use_context(cx).unwrap();
    let cell = move || game.with(|g| g.get(pos));

    let handle_mouse_down = move |e: MouseEvent| {
        set_mouse_down(Some(e.button()));
    };

    let send_mouse_action = move |e: MouseEvent| {
        let prev_mouse_button = mouse_down();
        if let Some(button) = prev_mouse_button {
            if button == e.button() {
                match button {
                    0 => set_game.update(|game| game.open_cell(pos)),
                    2 => set_game.update(|game| game.flag_cell(pos)),
                    _ => (),
                }
                return;
            }
        } else {
            return;
        }

        set_game.update(|game| game.chorded_open(pos));
    };

    let class = move || format!("Cell {}", cell().class);
    let style = format!(
        "grid-column-start: {}; grid-row-start: {};",
        pos.col + 1,
        pos.row + 1
    );

    view! { cx,
        <div
            class=class
            style=style
            on:mousedown=handle_mouse_down
            on:mouseup=send_mouse_action
            on:contextmenu=move |e| e.prevent_default()
        >
            {move || cell().icon}
        </div>
    }
}
