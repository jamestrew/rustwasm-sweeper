use crate::minesweeper::{GameState, Minesweeper, Pos};
use crate::ui::shared::GameUpdater;
use leptos::{ev::MouseEvent, *};
use leptos_meta::{Title, TitleProps};

use crate::ui::settings::*;

const CELL_SIZE: usize = 30;

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (game, set_game) = create_signal(cx, Minesweeper::new(9, 9, 10));
    let (active_pos, set_active_pos) = create_signal::<Vec<Pos>>(cx, Vec::new());

    let board_pos = move || game.with(|g| g.board.iter_pos().collect::<Vec<Pos>>());
    provide_context(
        cx,
        GameUpdater {
            set_game,
            set_active_pos,
        },
    );

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
        game.with(|game_state| match game_state.state {
            GameState::Win => log!("WINNER!!!"),
            GameState::Lose => log!("YOU LOSE :("),
            _ => (),
        });
    });

    view! { cx,
        <Title text="Minesweeper" />

        <div class="game">
            <div class="Board" style=style>
                <For
                    each=board_pos
                    key=|&pos| pos.key()
                    view=move |cx, pos| {
                        view! { cx,
                            <Cell game pos active_pos/>
                        }
                    }
                />
            </div>
            <SettingsPanel />
        </div>
    }
}

#[component]
pub fn Cell(
    cx: Scope,
    game: ReadSignal<Minesweeper>,
    pos: Pos,
    active_pos: ReadSignal<Vec<Pos>>,
) -> impl IntoView {
    let (mouse_down, set_mouse_down) = create_signal::<MouseButtons>(cx, MouseButtons::None);

    let GameUpdater {
        set_game,
        set_active_pos,
    } = use_context(cx).unwrap();

    let cell = move || game.with(|g| g.get_cell(pos));
    let active = move || active_pos.with(|ap| ap.contains(&pos) && !cell().kind.is_flagged());

    let handle_mouse_down = move |e: MouseEvent| {
        let buttons = MouseButtons::from_buttons(e.buttons());
        set_mouse_down(buttons);
        match buttons {
            MouseButtons::LClick => set_active_pos(vec![pos]),
            MouseButtons::LRClick => {
                set_active_pos(game.with(|g| g.unflagged_closed_neighbors(pos)))
            }
            _ => (),
        };
    };

    let send_mouse_action = move |_| {
        match mouse_down.get() {
            MouseButtons::LClick => set_game.update(|game| game.open_cell(pos)),
            MouseButtons::RClick => set_game.update(|game| game.flag_cell(pos)),
            MouseButtons::LRClick => set_game.update(|game| game.chorded_open(pos)),
            _ => (),
        };
        set_active_pos(Vec::new());
    };

    let class = move || {
        format!(
            "Cell {} {}",
            cell().class,
            if active() { "active" } else { "" }
        )
    };
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

#[derive(PartialEq, Copy, Clone)]
enum MouseButtons {
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
