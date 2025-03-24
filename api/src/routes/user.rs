use base64::{Engine, prelude::BASE64_STANDARD};
use rocket::{
    State, get,
    http::{CookieJar, Status},
    post,
    serde::json::Json,
};

use crate::{models::{response::Response, user::UserResponse}, services, utils::request_guard::HeaderGuard, AppState};

#[get("/user")]
pub async fn get_user(_guard: HeaderGuard) -> (Status, Json<HeaderGuard>) {
    (Status::Ok, Json(_guard))
}

#[get("/user/list")]
pub async fn get_users(_guard: HeaderGuard, 
    state: &State<AppState>) -> (Status, Json<Vec<UserResponse>>){
    services::user::get_users(state).await
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
    state: &State<AppState>,
    cookies: &'a CookieJar<'a>,
    code: &str,
) -> (Status, Json<Response>) {
    services::user::login(
        state,
        cookies,
        String::from_utf8(BASE64_STANDARD.decode(code).ok().unwrap())
            .ok()
            .unwrap(),
    )
    .await
}
