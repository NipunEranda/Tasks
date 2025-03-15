use base64::{Engine, prelude::BASE64_STANDARD};
use rocket::{http::{CookieJar, Status}, post, serde::json::Json, State};

use crate::{AppState, services};

#[post("/user/login/<code>")]
pub async fn login<'a>(state: &'a State<AppState>, cookies: &'a CookieJar<'a>, code: &str) -> (Status, Json<String>) {
    services::user::login(
        state,
        cookies,
        String::from_utf8(BASE64_STANDARD.decode(code).ok().unwrap())
            .ok()
            .unwrap(),
    ).await
}