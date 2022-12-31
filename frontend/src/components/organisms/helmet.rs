// use web_sys::HtmlHeadElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Helmet(props: &Props) -> Html {
    let document_head = gloo::utils::document().head().unwrap();

    create_portal(html! { {for props.children.iter()} }, document_head.into())
}
