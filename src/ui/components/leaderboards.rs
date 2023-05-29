use leptos::*;

#[component]
pub fn Leaderboards(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="Leaderboards">
            <Leaderboard difficulty="Beginner" />
            <Leaderboard difficulty="Intermediate" />
            <Leaderboard difficulty="Expert" />
        </div>
    }
}

#[component]
fn Leaderboard(cx: Scope, difficulty: &'static str) -> impl IntoView {
    view! { cx,
        <div class="Leaderboard">
            <h4>{difficulty}</h4>
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
