use yewdux::prelude::*;
use crate::api::AutoResponse;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct Store {
    // pub username: String,
    pub token: String,
    pub username: String,
}

pub fn login_reducer(auth_response: AutoResponse, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        // store.username = auth_response.data.username;
        store.token = auth_response.token;
        store.username = auth_response.username;
    });
}