use base64::{Engine, prelude::BASE64_STANDARD};
use rocket::{
    State, get,
    http::{CookieJar, Status},
    post
};

use crate::{models::response::Response, services, utils::request_guard::HeaderGuard, AppState};

#[get("/user")]
pub async fn get_user(_guard: HeaderGuard) -> (Status, String) {
    Response::ok(serde_json::to_string(&_guard).unwrap())
}

#[get("/user/list")]
pub async fn get_users(_guard: HeaderGuard, 
    state: &State<AppState>) -> (Status, String){
    services::user::get_users(state).await
}

#[get("/user/logout")]
pub async fn logout_user<'a>(cookies: &'a CookieJar<'a>) -> (Status, String) {
    for cookie in cookies.iter() {
        cookies.remove(cookie.clone());
    }

    Response::ok(String::from("Logged out successfully"))
}

#[post("/user/login/<code>")]
pub async fn login<'a>(
    state: &State<AppState>,
    cookies: &'a CookieJar<'a>,
    code: &str,
) -> (Status, String) {
    services::user::login(
        state,
        cookies,
        String::from_utf8(BASE64_STANDARD.decode(code).ok().unwrap())
            .ok()
            .unwrap(),
    )
    .await
}
