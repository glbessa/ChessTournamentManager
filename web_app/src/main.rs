use yew::prelude::*;

struct Counter {
    value: i64
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Counter {
        value: 0
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Counter {
                value: state.value + 1
            })
        })
    };

    html! {
        <div>
            <p>{ state.value }</p>
            <button onclick={onclick}>{ "+1" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new()
        .render();
}