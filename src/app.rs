use yew::prelude::*;
use crate::router::{AppRoute, switch};
use yew_router::prelude::{BrowserRouter, Switch, Link};


#[function_component(App)]
pub fn app() -> Html {
    
    html! {
        <BrowserRouter>
        <main>
            <Switch<AppRoute>
            render={Switch::render(switch)}
        />
        <ul>
        <li><Link<AppRoute> to={AppRoute::Index}>{ "click here to go home" }</Link<AppRoute>></li>
        <li><Link<AppRoute> to={AppRoute::About { id: 2 }}>{ "click here to go about" }</Link<AppRoute>></li>
        </ul>


        </main>
        </BrowserRouter>
    }
}


