pub mod minesweeper;
pub mod ui;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "hydrate")] {

        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            use ui::*;
            use leptos::*;

            // initializes logging using the `log` crate
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(move |cx| {
                view! { cx, <App/> }
            });
        }
    } else if #[cfg(feature = "ssr")] {
        use sqlx::{Pool, Sqlite};

        pub struct AppState {
            pub db_pool: Pool<Sqlite>,
        }

        pub fn register_server_functions() {
            use leptos::ServerFn;

            _ = ui::SaveScore::register();
            _ = ui::GetScores::register();
        }
    }
}
