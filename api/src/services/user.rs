use std::env;

use jsonwebtoken::{EncodingKey, Header, encode};
use reqwest::StatusCode;
use rocket::time::Duration;
use rocket::{
    State,
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
    time::OffsetDateTime,
};

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

    println!("{:?}", params);

    let client = reqwest::Client::new();
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await;

    println!("{:?}", response);

    let response = response.ok().unwrap();

    if response.status() == StatusCode::OK {
        let response_body: serde_json::Value = response.json().await.ok().unwrap();
        let response_body = response_body.as_object().unwrap();

        let user_response = client
            .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + response_body.get("access_token").unwrap().as_str().unwrap(),
            )
            .send()
            .await
            .ok();
        let user_response = user_response.unwrap();

        if user_response.status() == StatusCode::OK {
            let google_user = user_response.json::<GoogleUser>().await.ok().unwrap();

            let user = create_user(state, &google_user).await;

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
            cookies.add(cookie);

            return (Status::Ok, Json("true".to_string()));
        }
    }

    (Status::Unauthorized, Json("false".to_string()))
}
