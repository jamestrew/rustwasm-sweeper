use crate::minesweeper::{GameState, Minesweeper, Pos, SETTINGS};
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::*;
use leptos_meta::{Title, TitleProps};
use wasm_bindgen::JsValue;

use crate::ui::components::cell::*;
use crate::ui::components::scoreboard::*;
use crate::ui::components::settings::*;

use crate::ui::shared::{GameUpdater, MouseButtons, CELL_SIZE};

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (game, set_game) = create_signal(cx, Minesweeper::new(9, 9, 10));
    let (active_pos, set_active_pos) = create_signal::<Vec<Pos>>(cx, Vec::new());
    let (mouse_down, set_mouse_down) = create_signal::<MouseButtons>(cx, MouseButtons::None);
    let (setting, set_setting) = create_signal(cx, SETTINGS[0]);
    let (time, set_time) = create_signal(cx, 0);

    let game_state = store_value(cx, game.with(|g| g.state));
    let interval = store_value::<Option<Result<IntervalHandle, JsValue>>>(cx, None);

    let board_pos = move || game.with(|g| g.board.iter_pos().collect::<Vec<Pos>>());
    provide_context(
        cx,
        GameUpdater {
            game,
            set_game,
            setting,
            set_setting,
            time,
            set_time,
        },
    );

    create_effect(cx, move |_| {
        let state = game.with(|g| g.state);
        if game_state() == state {
            return;
        }

        game_state.update_value(|gs| *gs = state);
        log!("create_effect runs {:?}", game_state());
        if game_state() == GameState::Playing {
            let int = set_interval(
                move || set_time.update(|time| *time += 1),
                std::time::Duration::from_secs(1),
            );
            interval.update_value(|i| *i = Some(int));
        } else {
            if interval().is_some() {
                interval().unwrap().unwrap().clear();
            }
            if game_state() == GameState::Unstarted {
                set_time.set(0);
            }
        }
        // else if game_state() == GameState::Unstarted {
        //     set_time.set(0);
        // }
    });

    let style = move || {
        game.with(|g| {
            format!(
                "height: {}px; width: {}px",
                g.board.height as usize * CELL_SIZE,
                g.board.width as usize * CELL_SIZE
            )
        })
    };

    view! { cx,
        <Title text="Minesweeper" />

        <div class="game">
            <Scoreboard />
            <div class="Board" style=style>
                <For
                    each=board_pos
                    key=|&pos| pos.key()
                    view=move |cx, pos| {
                        view! { cx,
                            <Cell
                                pos
                                active_pos
                                set_active_pos
                                mouse_down
                                set_mouse_down
                            />
                        }
                    }
                />
            </div>
            <SettingsPanel />
        </div>
    }
}
