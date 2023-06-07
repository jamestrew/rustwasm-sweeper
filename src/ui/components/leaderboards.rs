use crate::minesweeper::Difficulty;
use crate::ui::components::game::LeaderboardScores;
use leptos::*;

#[component]
pub fn Leaderboards(cx: Scope, scores: ReadSignal<LeaderboardScores>) -> impl IntoView {
    view! { cx,
        <div class="Leaderboards">
            <Leaderboard difficulty=Difficulty::Beginner scores />
            <Leaderboard difficulty=Difficulty::Intermediate scores />
            <Leaderboard difficulty=Difficulty::Expert scores />
        </div>
    }
}

#[component]
fn Leaderboard(
    cx: Scope,
    difficulty: Difficulty,
    scores: ReadSignal<LeaderboardScores>,
) -> impl IntoView {

    view! { cx,
        <div class="Leaderboard">
            <h4>{difficulty.to_string()}</h4>
            <table>
                <For
                    each=move || scores.with(|s| s.diff_scores(&difficulty))
                    key=|score| score.time
                    view=move |cx, score| {
                        view! { cx,
                            <tr>
                                // <td style="width: 1em; text-align: left">{1}</td>
                                <td style="width: 8em">{score.name}</td>
                                <td style="width: 2em; text-align: right">{score.time}</td>
                            </tr>
                        }
                    }
                />
            </table>
        </div>
    }
    /*

    <div className="Leaderboard">
      <h4>{difficulty}</h4>
      <table>
        <tbody>
          {scores?.times
            .filter((_, idx) => idx < 5)
            .map((score, idx) => (
              <tr key={idx}>
                <td style={{ width: "1em", textAlign: "left" }}>{`${idx + 1}.`}</td>
                <td style={{ width: "8em" }}>{truncateStr(score.name, 15)}</td>
                <td style={{ width: "2em", textAlign: "right" }}>{score.time}</td>
              </tr>
            ))}
        </tbody>
      </table>
    </div>
    */
}
