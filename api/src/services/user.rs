use std::env;

use jsonwebtoken::{EncodingKey, Header, encode};
use mongodb::bson::doc;
use mongodb::Collection;
use reqwest::StatusCode;
use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;
use rocket::time::Duration;
use rocket::{
    State,
    http::{Cookie, CookieJar, Status},
    time::OffsetDateTime,
};

use crate::models::user::{User, UserResponse};
use crate::{
    AppState,
    models::{claims::Claims, user::GoogleUser},
};

use super::db::user::create_user;

pub async fn login<'a>(
    state: &State<AppState>,
    cookies: &'a CookieJar<'a>,
    code: String,
) -> (Status, Json<String>) {
    let domain = env::var("AUTH_DOMAIN").ok().unwrap();
    let params = [
        ("code", code),
        ("client_id", env::var("GOOGLE_CLIENT_ID").ok().unwrap()),
        ("client_secret", env::var("GOOGLE_SECRET").ok().unwrap()),
        (
            "redirect_uri",
            env::var("GOOGLE_REDIRECT_URI").ok().unwrap(),
        ),
        ("grant_type", "authorization_code".to_string()),
    ];

    let client = reqwest::Client::new();
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await.ok();

    let response = response.unwrap();

    if response.status() == StatusCode::OK {
        let response_body: serde_json::Value = response.json().await.ok().unwrap();
        let response_body = response_body.as_object().unwrap();

        let token = response_body.get("access_token").unwrap().as_str().unwrap();

        let user_response = client
            .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + token,
            )
            .send()
            .await
            .ok();
        let user_response = user_response.unwrap();

        if user_response.status() == StatusCode::OK {
            let mut google_user = user_response.json::<GoogleUser>().await.ok().unwrap();

            let user = create_user(state, &google_user).await;
            google_user.id = user._id.to_hex();

            let mut claims = Claims::new(google_user);
            claims._set_role(user.role);

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(env::var("JWT_KEY").ok().unwrap().as_bytes()),
            );

            let mut cookie = Cookie::new("tak", token.ok().unwrap().to_string());
            let mut now = OffsetDateTime::now_utc();
            now += Duration::days(1);

            cookie.set_http_only(true);
            cookie.set_secure(false);
            cookie.set_path("/");
            cookie.set_same_site(None);
            cookie.set_expires(now);
            cookie.set_domain(if domain == "" {String::from("localhost")} else {domain});
            cookies.add(cookie);

            return (Status::Ok, Json("true".to_string()));
        }
    }

    (Status::Unauthorized, Json("false".to_string()))
}

pub async fn get_users(state: &State<AppState>) -> (Status, Json<Vec<UserResponse>>) {
    let mut users: Vec<UserResponse> = Vec::new();
    let collection: Collection<User> = get_collection(state, "user").await;
    let result = collection.find(doc! {}).await;

    let cursor = match result {
        Ok(cursor) => cursor,
        Err(_) => return (Status::Ok, Json(vec![])),
    };

    cursor
        .try_collect()
        .await
        .unwrap_or(vec![])
        .iter()
        .for_each(|user| {
            users.push(UserResponse::copy(user));
        });

    (Status::Ok, Json(users))
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<User> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("tasks");
    db.collection::<User>(collection)
}
