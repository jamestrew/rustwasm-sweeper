mod game;
mod settings;
mod shared;

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
                    <Route path="counters" view=|cx| view! { cx, <Counters /> }/>
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

const MANY_COUNTERS: usize = 1000;

type CounterHolder = Vec<(usize, (ReadSignal<i32>, WriteSignal<i32>))>;

#[derive(Copy, Clone)]
struct CounterUpdater {
    set_counters: WriteSignal<CounterHolder>,
}

#[component]
pub fn Counters(cx: Scope) -> impl IntoView {
    let (next_counter_id, set_next_counter_id) = create_signal(cx, 0);
    let (counters, set_counters) = create_signal::<CounterHolder>(cx, vec![]);
    provide_context(cx, CounterUpdater { set_counters });

    let add_counter = move |_| {
        let id = next_counter_id();
        let sig = create_signal(cx, 0);
        set_counters.update(move |counters| counters.push((id, sig)));
        set_next_counter_id.update(|id| *id += 1);
    };

    let add_many_counters = move |_| {
        let next_id = next_counter_id();
        let new_counters = (next_id..next_id + MANY_COUNTERS).map(|id| {
            let signal = create_signal(cx, 0);
            (id, signal)
        });

        set_counters.update(move |counters| counters.extend(new_counters));
        set_next_counter_id.update(|id| *id += MANY_COUNTERS);
    };

    let clear_counters = move |_| {
        set_counters.update(|counters| counters.clear());
    };

    view! { cx,
        <>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <button on:click=add_many_counters>
                {format!("Add {MANY_COUNTERS} Counters")}
            </button>
            <button on:click=clear_counters>
                "Clear Counters"
            </button>
            <p>
                "Total: "
                <span>{move ||
                    counters.get()
                        .iter()
                        .map(|(_, (count, _))| count())
                        .sum::<i32>()
                        .to_string()
                }</span>
                " from "
                <span>{move || counters().len().to_string()}</span>
                " counters."
            </p>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    view=move |cx, (id, (value, set_value)): (usize, (ReadSignal<i32>, WriteSignal<i32>))| {
                        view! { cx,
                            <Counter id value set_value/>
                        }
                    }
                />
            </ul>
        </>
    }
}

#[component]
fn Counter(
    cx: Scope,
    id: usize,
    value: ReadSignal<i32>,
    set_value: WriteSignal<i32>,
) -> impl IntoView {
    let CounterUpdater { set_counters } = use_context(cx).unwrap();

    let input = move |ev| set_value(event_target_value(&ev).parse::<i32>().unwrap_or_default());

    // just an example of how a cleanup function works
    // this will run when the scope is disposed, i.e., when this row is deleted
    on_cleanup(cx, || log::debug!("deleted a row"));

    view! { cx,
        <li style="color: red">
            <button on:click=move |_| set_value.update(move |value| *value -= 1)>"-1"</button>
            <input type="text"
                prop:value={value}
                on:input=input
            />
            <span>{move || {
                log!("rendering");
                value
            }}</span>
            <button on:click=move |_| set_value.update(move |value| *value += 1)>"+1"</button>
            <button on:click=move |_| set_counters.update(move |counters| counters.retain(|(counter_id, _)| counter_id != &id))>"x"</button>
        </li>
    }
}
