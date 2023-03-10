use yew::{html, Html};
use yew_router::Routable;

use crate::pages::{BlogPage, HomePage};

#[derive(Routable, PartialEq, Eq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage />},
        Route::Blog => html! { <BlogPage />},
    }
}
