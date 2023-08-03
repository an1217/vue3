mod components;
mod pages;
mod router;
mod store;
mod api;

use components::organisms::navbar::Navbar;
use router::{switch, Route};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

#[derive(Serialize, Deserialize)]
struct My {
    username: String,
    password: String,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
     <>

         <BrowserRouter>
         <Navbar/ >
             <Switch<Route> render={switch} />
         </BrowserRouter>
     </>
    }
}
