use crate::pages::{create_accounts::CreateAccount, home::Home, login::Login};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/create_accounts")]
    CreateAccount,
    #[at("/login")]
    Login
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::CreateAccount => html! {<CreateAccount/>},
        Route::Login => html! {<Login />},
    }
}
