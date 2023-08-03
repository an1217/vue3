use std::ops::Deref;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use stylist::{css, yew::styled_component};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;
use crate::{components::atoms::{bb_button::BBButton, bb_input::{BBTextInput, InputType}}, store::{Store, login_reducer}, api, router::Route};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AutoResponse {
    pub token: String,
    pub username: String,
}

#[derive(Default, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub phone: String,
    pub nickname: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub onsubmit: Callback<User>
}

#[styled_component]
pub fn AccounFrom(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
        
    "#
    );
    let state = use_state(|| User::default());
   
    let username_onchange = {
        let state = state.clone();  
         Callback::from(move |username: String| {
            let mut user = state.deref().clone();
            user.username = username;
            state.set(user);
        })
    };

    let password_onchange = {
        let state = state.clone();  
        Callback::from(move |password: String| {
            let mut user = state.deref().clone();
            user.password = password;
            state.set(user);
        })
    };

    let phone_onchange = {
        let state = state.clone();  
        Callback::from(move |phone: String| {
            let mut user = state.deref().clone();
            user.password = phone;
            state.set(user);
        })
    };

    let onsubmit = {
        let onsubmit_prop = props.onsubmit.clone();
        let state = state.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let user = state.deref().clone();
            onsubmit_prop.emit(user); 
        })
    };

    html! {
        <form {onsubmit}>
            <BBTextInput  label={"用户名"} placeholder={"请输入用户名"} class="input" input_type={InputType::Normal} onchange={username_onchange}/>
            <BBTextInput  label={"密码"} placeholder={"请输入密码"} class="input" input_type={InputType::Password} onchange={password_onchange}/>
            <BBTextInput  label={"手机号"} placeholder={"请输入手机号码"} class="input" input_type={InputType::Normal} onchange={phone_onchange}/>

            <BBButton data_test="submit"  label="Creat Account" />
            
        </form>
    }

}