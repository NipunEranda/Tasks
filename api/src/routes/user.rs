use base64::{Engine, prelude::BASE64_STANDARD};
use rocket::{State, http::Status, post, serde::json::Json};

use crate::{AppState, services};

#[post("/user/login/<code>")]
pub async fn login(state: &State<AppState>, code: &str) -> (Status, Json<String>) {
    services::user::login(
        state,
        String::from_utf8(BASE64_STANDARD.decode(code).ok().unwrap())
            .ok()
            .unwrap(),
    ).await
}
