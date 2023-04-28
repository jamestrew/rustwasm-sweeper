use crate::minesweeper::Pos;
use crate::ui::shared::{GameUpdater, MouseButtons};
use leptos::{ev::MouseEvent, *};

#[component]
pub fn Cell(
    cx: Scope,
    pos: Pos,
    active_pos: ReadSignal<Vec<Pos>>,
    set_active_pos: WriteSignal<Vec<Pos>>,
    mouse_down: ReadSignal<MouseButtons>,
    set_mouse_down: WriteSignal<MouseButtons>,
) -> impl IntoView {
    let GameUpdater { game, set_game, .. } = use_context(cx).unwrap();

    let cell = move || game.with(|g| g.get_cell(pos));
    let active = move || active_pos.with(|ap| ap.contains(&pos) && !cell().kind.is_flagged());

    let handle_mouse_down = move |e: MouseEvent| {
        let buttons = MouseButtons::from_buttons(e.buttons());
        set_mouse_down(buttons);
        match buttons {
            MouseButtons::LClick => set_active_pos(vec![pos]),
            MouseButtons::LRClick => set_active_pos(game.with(|g| g.chorded_cells(pos))),
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
        set_mouse_down(MouseButtons::None);
    };

    let handle_mouse_enter = move |_| match mouse_down.get() {
        MouseButtons::None => (),
        MouseButtons::LClick => set_active_pos(vec![pos]),
        MouseButtons::LRClick => set_active_pos(game.with(|g| g.chorded_cells(pos))),
        _ => (),
    };

    let handle_mouse_leave = move |_| {
        set_active_pos(vec![]);
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
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
            on:contextmenu=move |e| e.prevent_default()
        >
            {move || cell().icon}
        </div>
    }
}
