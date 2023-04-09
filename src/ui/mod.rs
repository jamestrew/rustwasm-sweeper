mod game;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use rand::Rng;

use game::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Game /> }/>
                    <Route path="demo" view=|cx| view! { cx, <HomePage title=String::from("Demo")/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope, title: String) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let increment = move |_| set_count.update(|count| *count += 1);
    let reset = move |_| set_count.update(|count| *count = 0);
    let increment_random = move |_| {
        log!("incrementing randomly");
        set_count.update(|count| {
            let val = rand::thread_rng().gen_range(0..9);
            *count += val;
        })
    };

    view! { cx,
        <Title text={title} />
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=increment>"Click Me: " {count}</button>
        <button on:click=increment_random>"Increment Random"</button>
        <button on:click=reset>"Reset"</button>
    }
}
