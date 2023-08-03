use yew::prelude::*;
use stylist::{css, yew::styled_component};

#[styled_component]
pub fn Login() -> Html {
    let stylesheet = css!(r#""#);

    html! {
        <h1 class = {stylesheet}>
            {"Login"}
        </h1>
    }
}   
