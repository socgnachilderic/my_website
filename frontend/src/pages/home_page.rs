use yew::prelude::*;

// use crate::components::organisms::helmet::Helmet;

#[function_component]
pub fn HomePage() -> Html {
    let fallback = html! { <div>{"Loading.."}</div> };

    html! {
        <>
            // <Helmet>
            //     <title>{"Home Page"}</title>
            // </Helmet>
            <Suspense {fallback}>
                <p>{"Childeric says <Hello world>"}</p>
            </Suspense>
        </>
    }
}
