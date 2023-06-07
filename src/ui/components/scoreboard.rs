use leptos::*;

use crate::{
    minesweeper::{GameState, Minesweeper},
    ui::shared::GameUpdater,
};

#[component]
pub fn Scoreboard(cx: Scope) -> impl IntoView {
    let GameUpdater {
        game,
        time,
        setting,
        ..
    } = use_context(cx).unwrap();

    let flags_remaining = move || {
        game.with(|g| {
            setting().mine_count
                - g.board
                    .iter()
                    .flatten()
                    .filter(|kind| kind.is_flagged())
                    .count()
        })
    };

    view! { cx,
        <div class="Scoreboard">
            <div class="Counter">{ flags_remaining }</div>
            <MinesweeperGuy />
            <div class="Counter">{ time }</div>
        </div>
    }
}

#[component]
fn MinesweeperGuy(cx: Scope) -> impl IntoView {
    let GameUpdater {
        game,
        set_game,
        setting,
        ..
    } = use_context(cx).unwrap();

    let guy = move || game.with(|g| g.state.as_emoji());

    let restart_game = move |_| set_game(Minesweeper::from_setting(setting.get()));

    view! { cx,
        <div class="MinesweeperGuy" on:click=restart_game>
            {guy}
        </div>
    }
}

impl GameState {
    fn as_emoji(&self) -> &'static str {
        match self {
            GameState::Unstarted | GameState::Playing => "😇",
            GameState::Win => "😎",
            GameState::Lose => "💀",
        }
    }
}
