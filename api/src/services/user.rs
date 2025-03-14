use std::env;

use reqwest::StatusCode;
use rocket::{State, http::Status, serde::json::Json};

use crate::{models::user::GoogleAuth, AppState};

pub async fn login(state: &State<AppState>, code: String) -> (Status, Json<String>) {
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
        .await
        .ok();

    let response = response.unwrap();

    if response.status() == StatusCode::OK {
        let response_body: serde_json::Value = response.json().await.ok().unwrap();
        let response_body = response_body.as_object().unwrap();

        let google_auth = GoogleAuth::new(response_body.get("access_token").unwrap().to_string(), response_body.get("refresh_token").unwrap().to_string(), response_body.get("expires_in").unwrap().to_string(), response_body.get("scope").unwrap().to_string(), response_body.get("token_type").unwrap().to_string(), response_body.get("id_token").unwrap().to_string());
        println!("{:?}", google_auth);

        return (Status::Accepted, Json("true".to_string()));
    }

    (Status::Unauthorized, Json("false".to_string()))
}
