use yew::prelude::*;
use yew::ServerRenderer;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use routes::*;

mod pages;
mod routes;

pub async fn render_app(uri: String) -> String {
    ServerRenderer::<ServerApp>::with_props(move || ServerAppProps { url: uri.into() })
        .render()
        .await
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component]
fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.url);

    html! {
        <Router {history}>
            <Switch<Route> render={switch} />
        </Router>
    }
}

#[derive(Properties, PartialEq, Debug)]
struct ServerAppProps {
    pub url: AttrValue,
}
