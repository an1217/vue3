use std::ops::Deref;

use super::super::components::atoms::{
    bb_button::BBButton,
    bb_input::{BBTextInput, InputType},
};
use crate::{api,router::Route, store::{Store,  login_reducer}};
use gloo::console::log;
use serde::{Serialize, Deserialize};
use stylist::yew::styled_component;
use yew::{prelude::*, platform::spawn_local};
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AutoResponse {
    pub token: String,
    pub username: String,
}

#[derive(Default)]
pub struct State {
    pub username: String,
    pub password: String,
}

#[styled_component(CreateAccount)]
pub fn create_account() -> Html {
    let stylesheet = css!(
        r#"
        section {
            display: flex;
            justify-content: center;
            flex-direction: column;
        }
        .input {
            width: 75%
        }
        
    "#
    );
    let username_state = use_state(String::default);
    let password_state = use_state(String::default);
    let onsubmit ={ 
        let username_state = username_state.clone();
        let password_state = password_state.clone();
        let history = use_navigator().unwrap();
        let (_state, dispatch) = use_store::<Store>();
        Callback::from(move |event: SubmitEvent| {
            let username_state = username_state.clone();
            let password_state = password_state.clone();
            let navigator = history.clone();
            let store = dispatch.clone();
            
            event.prevent_default();
            spawn_local(async move {
                let result = api::login_account(username_state.deref().to_owned(), password_state.deref().to_string()).await;
                login_reducer(result,store);              
                navigator.push(&Route::Home);  
            });
        })
    };

    let username_onchange = {
        let state = username_state.clone();  
         Callback::from(move |username: String| {
            state.set(username);
        })
    };

    let password_onchange = {
        let state = password_state.clone();  
        Callback::from(move |password: String| {
            state.set(password);
        })
    };

    html! {
        <>
            <div class={stylesheet}>
                <h1>{"create account"}</h1>
                <section>
                    <div>
                        <form {onsubmit}>
                            <BBTextInput label={"用户名"} placeholder={"请输入用户名"} class="input" input_type={InputType::Normal} onchange={username_onchange}/>
                            <BBTextInput label={"密码"} placeholder={"请输入密码"} class="input" input_type={InputType::Password} onchange={password_onchange}/>
                            <BBButton data_test="submit"  label="Creat Account" />
                
                        </form>
                    </div>
                </section>
            </div>
        </>
    }
}
