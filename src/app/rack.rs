use super::game::*;
use leptos::*;

#[component]
pub fn Rack(rack_signal: RwSignal<Vec<Tile>>, bag_signal: RwSignal<Vec<Tile>>) -> impl IntoView {
    // let global_state = use_context::<GlobalState>().unwrap();

    let draw_rack = move || {
        if !rack_signal().is_empty() {
            rack_signal()
            .into_iter()
            // .rev()
            .map(|t| {
                view! {
                    <li class="inline-block"><button class="tile-bag">
                    {t.0}
                    <sub class="text-xs" class=("hidden", move || t.1 == 0)>{t.1}</sub></button></li>
                }
            })
            .collect_view()
        } else {
            view! {}.into_view()
        }
    };

    // let draw_true = move || {
    //     if global_state.global_toggle.get() {
    //         view! {{draw_rack}}.into_view()
    //     } else {
    //         view! {}.into_view()
    //     }
    // };

    let draw_bag = move || {
        if !bag_signal().is_empty() {
            bag_signal()
                .into_iter()
                .map(|t| {
                    view! {
                        <li class="inline-block dark:text-zinc-300">{t.0}
                        <sub class="text-xs" class=("hidden", move || t.1 != 0)>{t.1}</sub></li>
                    }
                })
                .collect_view()
        } else {
            view! {}.into_view()
        }
    };

    view! {
        <ul>{draw_rack}</ul>

        <p>{move || bag_signal().len()} " lettres dans le sac."</p>
        <ul>{draw_bag}</ul>
        // <ul><p>Sac:</p> {draw_bag}</ul>
    }
}
