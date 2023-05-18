use leptos::*;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_web::web;
        use sqlx::{Pool, Sqlite};

        pub struct AppState {
            pub db_pool: Pool<Sqlite>,
        }

        pub fn register_server_functions() {
            _ = Testing::register();
        }
    }
}

#[server(Testing, "/api")]
pub async fn testing(cx: Scope, txt: String) -> Result<String, ServerFnError> {
    println!("testing called");
    let _app_state = use_context::<web::Data<AppState>>(cx);
    Ok(txt.to_uppercase())
}
