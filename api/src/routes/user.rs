use base64::{Engine, prelude::BASE64_STANDARD};
use rocket::{
    State, get,
    http::{Cookie, CookieJar, Status},
    post,
    serde::json::Json,
};

use crate::{AppState, services, utils::request_guard::HeaderGuard};

#[get("/user")]
pub async fn get_user(_guard: HeaderGuard) -> (Status, Json<HeaderGuard>) {
    (Status::Ok, Json(_guard))
}

#[get("/user/logout")]
pub async fn logout_user<'a>(cookies: &'a CookieJar<'a>) -> (Status, Json<String>) {
    for cookie in cookies.iter() {
        cookies.remove(cookie.clone());
    }

    (Status::Ok, Json(String::from("Logged Out Successfully")))
}

#[post("/user/login/<code>")]
pub async fn login<'a>(
    state: &'a State<AppState>,
    cookies: &'a CookieJar<'a>,
    code: &str,
) -> (Status, Json<String>) {
    services::user::login(
        state,
        cookies,
        String::from_utf8(BASE64_STANDARD.decode(code).ok().unwrap())
            .ok()
            .unwrap(),
    )
    .await
}
