use yew::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    let fallback = html! { <div>{"Loading.."}</div> };

    html! {
        <Suspense {fallback}>
            <p>{"Childeric says <Hello world>"}</p>
        </Suspense>
    }
}
