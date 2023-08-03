use reqwasm::http::Request;
use serde_json::json;
use serde::{Deserialize, Serialize};

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


 pub async fn login_account(username: String, password: String) -> AutoResponse {
    Request::post("http://127.0.0.1:9022/user/login")
                .header("Content-Type", "application/json")
                .body(json!({
                    "username": username,
                    "password": password,

                    }).to_string()
                ).send()
                .await
                .unwrap()
                .json::<AutoResponse>()
                .await
                .unwrap()
}

// pub async fn login_account(username: String, password: String) -> AutoResponse {
    
// }