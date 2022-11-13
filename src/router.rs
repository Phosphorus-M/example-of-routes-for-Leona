use yew::{Html, html};
use yew_router::prelude::*;

#[derive(Clone, Debug, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/resources")]
    Resources,
    #[at("/events")]
    Events,
    #[at("/about/:id")]
    About { id: u8 },
    #[at("/")]
    Index,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Resources => html! { <h1>{"Resources"}</h1> },
        AppRoute::Events => html! { <h1>{"Events"}</h1> },
        AppRoute::About { id } => html! { <h1>{"About"} {id}</h1> },
        AppRoute::Index => {
            html! { <h1>{"Index"} </h1> }
        }
    }
}