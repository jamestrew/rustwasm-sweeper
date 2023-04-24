use crate::minesweeper::Minesweeper;
use crate::ui::utils::GameUpdater;
use leptos::*;

#[component]
pub fn OptionsPanel(cx: Scope) -> impl IntoView {
    let GameUpdater { set_game } = use_context(cx).unwrap();
    let (setting, set_setting) = create_signal(cx, &OPTIONS[0]);

    let mode_select = move |ev, setting| {
        if event_target_checked(&ev) {
            set_setting(setting);
        }
    };

    let new_game = move |_| {
        let setting = setting.get();
        set_game(Minesweeper::new(setting.height, setting.width, setting.mine_count));
    };

    view! { cx,
        <>
            <div class="Options">
                <table id="option-table">
                  <thead>
                    <tr>
                      <th></th>
                      <th></th>
                      <th>{ "Width" }</th>
                      <th>{ "Height" }</th>
                      <th>{ "Mines" }</th>
                    </tr>
                  </thead>
                  <tbody>
                    <For
                        each=move || OPTIONS
                        key=|&opt| opt.difficulty.to_string()
                        view=move |cx, opt| {
                            view!{ cx,
                                <tr>
                                    <td>
                                        <input
                                            type="radio"
                                            name="mode"
                                            prop:value={opt.difficulty}
                                            prop:checked={move || setting.with(|&diff| diff.difficulty == opt.difficulty)}
                                            on:change=move |ev| mode_select(ev, opt)
                                        />
                                    </td>
                                    <td>{opt.difficulty.to_string()}</td>
                                    <td>{opt.width}</td>
                                    <td>{opt.height}</td>
                                    <td>{opt.mine_count}</td>
                                </tr>
                            }
                        }
                    />
                  </tbody>
                </table>
            </div>
            <button on:click=new_game>{"New Game"}</button>
        </>
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
    Custom,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Beginner => write!(f, "Beginner"),
            Difficulty::Intermediate => write!(f, "Intermediate"),
            Difficulty::Expert => write!(f, "Expert"),
            Difficulty::Custom => write!(f, "Custom"),
        }
    }
}

impl IntoProperty for Difficulty {
    fn into_property(self, _: Scope) -> Property {
        Property::Value(self.to_string().into())
    }
}

impl IntoAttribute for Difficulty {
    fn into_attribute(self, _: Scope) -> Attribute {
        Attribute::String(self.to_string())
    }

    #[inline]
    fn into_attribute_boxed(self: Box<Self>, cx: Scope) -> Attribute {
        self.into_attribute(cx)
    }
}

#[derive(Clone, Copy)]
struct Setting {
    pub difficulty: Difficulty,
    pub width: u8,
    pub height: u8,
    pub mine_count: usize,
}

const OPTIONS: &[Setting] = &[
    Setting {
        difficulty: Difficulty::Beginner,
        width: 9,
        height: 9,
        mine_count: 10,
    },
    Setting {
        difficulty: Difficulty::Intermediate,
        width: 16,
        height: 16,
        mine_count: 40,
    },
    Setting {
        difficulty: Difficulty::Expert,
        width: 30,
        height: 16,
        mine_count: 99,
    },
    Setting {
        difficulty: Difficulty::Custom,
        width: 9,
        height: 9,
        mine_count: 10,
    },
];
