use crate::minesweeper::Minesweeper;
use leptos::*;
use leptos_meta::{Title, TitleProps};

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let mut game = Minesweeper::new(9, 9, 10);
    game.create_mines(None);
    log!("{}", game);

    view! { cx,
        <Title text="Minesweeper" />
        <p>{game.to_string()}</p>
    }
}
