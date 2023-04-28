use crate::minesweeper::{Minesweeper, Pos, SETTINGS};
use leptos::*;
use leptos_meta::{Title, TitleProps};

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

    let board_pos = move || game.with(|g| g.board.iter_pos().collect::<Vec<Pos>>());
    provide_context(
        cx,
        GameUpdater {
            game,
            set_game,
            setting,
            set_setting,
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
