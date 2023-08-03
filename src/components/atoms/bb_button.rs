use stylist::yew::styled_component;
use yew::prelude::*;
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub data_test: String,
    pub label: String,
}

#[styled_component]
pub fn BBButton(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
            width: 100%;
            font-size: 36px;
            margin: 10px 0;
            background-color: red;
        "#
    );
    html! {
        <button data_test={props.data_test.clone()} class={stylesheet}>{&props.label}</button>
    }
}
