use std::borrow::Cow;

use crate::minesweeper::{Difficulty, Minesweeper, CUSTOM, SETTINGS};
use crate::ui::shared::GameUpdater;
use leptos::*;

enum SettingField {
    Width,
    Height,
    MineCount,
}

impl IntoProperty for Difficulty {
    fn into_property(self, _: Scope) -> Property {
        Property::Value(self.to_string().into())
    }
}

impl IntoAttribute for Difficulty {
    fn into_attribute(self, _: Scope) -> Attribute {
        Attribute::String(Cow::Owned(self.to_string()))
    }

    #[inline]
    fn into_attribute_boxed(self: Box<Self>, cx: Scope) -> Attribute {
        self.into_attribute(cx)
    }
}

#[component]
pub fn SettingsPanel(cx: Scope) -> impl IntoView {
    let GameUpdater {
        set_game,
        setting,
        set_setting,
        ..
    } = use_context(cx).unwrap();
    let (custom_setting, set_custom_setting) = create_signal(cx, CUSTOM);

    let mode_select = move |ev, setting| {
        if event_target_checked(&ev) {
            set_setting(setting);
        }
    };

    let update_custom_field = move |ev, field| {
        let setting = custom_setting.get();
        match field {
            SettingField::Width => {
                let num = event_target_value(&ev)
                    .parse::<u8>()
                    .unwrap_or(setting.width);
                set_custom_setting.update(|setting| setting.width = num);
            }
            SettingField::Height => {
                let num = event_target_value(&ev)
                    .parse::<u8>()
                    .unwrap_or(setting.height);
                set_custom_setting.update(|setting| setting.height = num);
            }
            SettingField::MineCount => {
                let num = event_target_value(&ev)
                    .parse::<usize>()
                    .unwrap_or(setting.mine_count);
                if num < (setting.width * setting.height) as usize {
                    set_custom_setting.update(|setting| setting.mine_count = num)
                }
            }
        }
        set_setting(custom_setting.get());
    };

    let new_game = move |_| set_game(Minesweeper::from_setting(setting.get()));

    view! { cx,
        <>
            <div class="Settings">
                <table id="settings-table">
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
                        each=move || SETTINGS
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
                    <tr>
                        <td>
                            <input
                                type="radio"
                                name="mode"
                                prop:value={CUSTOM.difficulty}
                                prop:checked={move || setting.with(|&diff| diff.difficulty == CUSTOM.difficulty)}
                                on:change=move |ev| mode_select(ev, custom_setting.get())
                            />
                        </td>
                        <td>{CUSTOM.difficulty.to_string()}</td>
                        <td>
                            <input
                                type="number"
                                class="custom-input"
                                prop:min="1"
                                prop:max="255"
                                prop:value={move || custom_setting().width }
                                on:change=move |ev| update_custom_field(ev, SettingField::Width)
                            />
                        </td>
                        <td>
                            <input
                                type="number"
                                class="custom-input"
                                prop:min="1"
                                prop:max="255"
                                prop:value={move || custom_setting().height }
                                on:change=move |ev| update_custom_field(ev, SettingField::Height)
                            />
                        </td>
                        <td>
                            <input
                                type="number"
                                class="custom-input"
                                prop:min="1"
                                prop:max={move || ((custom_setting().width * custom_setting().height) - 1).to_string() }
                                prop:value={move || custom_setting().mine_count }
                                on:change=move |ev| update_custom_field(ev, SettingField::MineCount)
                            />
                        </td>
                    </tr>
                  </tbody>
                </table>
            </div>
            <button on:click=new_game>{"New Game"}</button>
        </>
    }
}
