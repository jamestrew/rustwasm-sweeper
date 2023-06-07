use std::collections::HashMap;

use crate::minesweeper::Difficulty;
use crate::minesweeper::{GameState, Minesweeper, Pos, SETTINGS};
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::*;
use leptos_meta::Title;
use wasm_bindgen::JsValue;

use crate::ui::components::cell::*;
use crate::ui::components::leaderboards::*;
use crate::ui::components::scoreboard::*;
use crate::ui::components::settings::*;

use crate::ui::shared::{GameUpdater, MouseButtons, CELL_SIZE};

const TIMER_MAX: u16 = 999;

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let (game, set_game) = create_signal(cx, Minesweeper::new(9, 9, 10));
    let (active_pos, set_active_pos) = create_signal::<Vec<Pos>>(cx, Vec::new());
    let (mouse_down, set_mouse_down) = create_signal::<MouseButtons>(cx, MouseButtons::None);
    let (setting, set_setting) = create_signal(cx, SETTINGS[0]);
    let (time, set_time) = create_signal::<u16>(cx, 0);
    let (scores, set_scores) = create_signal::<LeaderboardScores>(cx, LeaderboardScores::default());

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
        },
    );

    spawn_local(async move {
        let scores = get_leaderboard_scores(cx).await.unwrap_or_default();
        set_scores.set(scores);
    });

    create_effect(cx, move |_| {
        let state = game.with(|g| g.state);
        if game_state() == GameState::Playing && time() >= TIMER_MAX && interval().is_some() {
            interval().unwrap().unwrap().clear();
        }
        if game_state() == state {
            return;
        }

        game_state.update_value(|gs| *gs = state);
        if game_state() == GameState::Playing {
            let int = set_interval_with_handle(
                move || set_time.update(|time| *time += 1),
                std::time::Duration::from_secs(1),
            );
            interval.update_value(|i| *i = Some(int));
        } else {
            if interval().is_some() {
                interval().unwrap().unwrap().clear();
            }
            match game_state() {
                GameState::Unstarted => set_time.set(0),
                GameState::Win => {
                    set_playername();
                    spawn_local(async move {
                        _ = save_player_score(
                            cx,
                            time.get_untracked(),
                            setting.get_untracked().difficulty,
                        )
                        .await;
                    });
                }
                _ => {}
            }
        }
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
            <Leaderboards scores />
        </div>
    }
}

impl Difficulty {
    pub fn id(&self) -> u8 {
        match self {
            Difficulty::Beginner => 1,
            Difficulty::Intermediate => 2,
            Difficulty::Expert => 3,
            Difficulty::Custom => 4,
        }
    }
}

#[server(SaveScore, "/api")]
pub async fn save_player_score(
    cx: Scope,
    time: u16,
    difficulty: Difficulty,
) -> Result<(), ServerFnError> {
    use crate::AppState;
    use actix_web::{web, HttpRequest};
    let difficulty_id = difficulty.id();
    let req =
        use_context::<HttpRequest>(cx).ok_or(ServerFnError::ServerError("no request".into()))?;

    let playername = req
        .cookie("playername")
        .map(|cookie| cookie.value().to_owned())
        .ok_or(ServerFnError::ServerError(
            "playername cookie field not set".into(),
        ))?;

    let app_state = req
        .app_data::<web::Data<AppState>>()
        .ok_or(ServerFnError::ServerError("no app state".into()))?;
    let db = &app_state.db_pool;

    _ = sqlx::query!("INSERT OR IGNORE INTO player (name) VALUES (?)", playername)
        .execute(db)
        .await;

    let player = sqlx::query!("SELECT id AS id FROM player WHERE name = ?", playername)
        .fetch_one(db)
        .await
        .map_err(|msg| ServerFnError::ServerError(msg.to_string()))?;

    if player.id.is_none() {
        return Err(ServerFnError::ServerError(
            "failed to fetch player id".into(),
        ));
    }

    let scores = sqlx::query!(
        "
    SELECT COUNT(*) AS 'count'
    FROM
        score AS s
        JOIN difficulty AS d ON s.difficulty_id = d.id
    WHERE
        s.time < ?
        AND d.id = ?
    ORDER BY s.time
    ",
        time,
        difficulty_id
    )
    .fetch_one(db)
    .await
    .map_err(|msg| ServerFnError::ServerError(msg.to_string()))?;

    if scores.count >= 10 {
        return Ok(());
    }

    _ = sqlx::query!(
        "
    INSERT INTO score (player_id, difficulty_id, time)
    VALUES (?, ?, ?)
    ",
        player.id,
        difficulty_id,
        time
    )
    .execute(db)
    .await
    .map_err(|msg| ServerFnError::ServerError(msg.to_string()))?;

    Ok(())
}

fn set_playername() {
    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    if is_playname_set(&doc.cookie().unwrap()) {
        return;
    }

    use wasm_bindgen::JsCast;
    let name = window()
        .prompt_with_message("Enter name to save score")
        .unwrap()
        .unwrap_or_default();

    if name.is_empty() {
        return;
    }

    _ = doc.set_cookie(format!("playername={}; SameSite=None; Secure", name).as_str());
}

fn is_playname_set(cookie: &str) -> bool {
    for item in cookie.split(";") {
        let mut item = item.split("=");
        if item.next() == Some("playername") {
            return true;
        }
    }
    return false;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Score {
    pub id: i64,
    pub name: String,
    pub time: i64,
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardScores(pub HashMap<Difficulty, Vec<Score>>);

impl LeaderboardScores {
    pub fn diff_scores(&self, difficulty: &Difficulty) -> Vec<Score> {
        // self.0[difficulty].clone()
        self.0.get(difficulty).unwrap_or(&Vec::new()).clone()
    }
}

impl Default for LeaderboardScores {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[server(GetScores, "/api")]
pub async fn get_leaderboard_scores(cx: Scope) -> Result<LeaderboardScores, ServerFnError> {
    use crate::AppState;
    use actix_web::{web, HttpRequest};

    let req =
        use_context::<HttpRequest>(cx).ok_or(ServerFnError::ServerError("no request".into()))?;

    let app_state = req
        .app_data::<web::Data<AppState>>()
        .ok_or(ServerFnError::ServerError("no app state".into()))?;
    let db = &app_state.db_pool;

    let mut scores = LeaderboardScores(HashMap::new());
    for difficulty in &[
        Difficulty::Beginner,
        Difficulty::Intermediate,
        Difficulty::Expert,
    ] {
        let diff_id = difficulty.id();
        let diff_score = sqlx::query_as!(
            Score,
            r#"
        SELECT s.id AS "id!", p.name AS "name!", s.time AS "time!"
        FROM
            score AS s
        INNER JOIN player AS p ON p.id = s.player_id
        INNER JOIN difficulty AS d ON d.id = s.difficulty_id
        WHERE
            d.id = ?
        ORDER BY
            s.time
        LIMIT 10;
        "#,
            diff_id
        )
        .fetch_all(db)
        .await
        .map_err(|msg| ServerFnError::ServerError(msg.to_string()))?;

        scores.0.insert(*difficulty, diff_score);
    }
    Ok(scores)
}
