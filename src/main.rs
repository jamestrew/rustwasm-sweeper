
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use dotenvy_macro::dotenv;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_start::ui::*;
    use leptos_start::{AppState, register_server_functions};
    use sqlx::SqlitePool;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    let db_url = dotenv!("DATABASE_URL");
    let pool = SqlitePool::connect(db_url).await.unwrap();
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("could not run SQLx migrations");

    register_server_functions();

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: pool.clone(),
            }))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .service(Files::new("/", site_root))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
