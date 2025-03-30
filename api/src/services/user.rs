use std::env;

use jsonwebtoken::{EncodingKey, Header, encode};
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use reqwest::StatusCode;
use rocket::futures::TryStreamExt;
use rocket::time::Duration;
use rocket::{
    State,
    http::{Cookie, CookieJar, Status},
    time::OffsetDateTime,
};

use crate::models::response::Response;
use crate::models::user::{User, UserResponse};
use crate::utils::tools;
use crate::{
    AppState,
    models::{claims::Claims, user::GoogleUser},
};

pub async fn login<'a>(
    state: &State<AppState>,
    cookies: &'a CookieJar<'a>,
    code: String,
) -> (Status, String) {
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
        .await
        .ok();

    let response = response.unwrap();

    if response.status() == StatusCode::OK {
        let response_body: serde_json::Value = response.json().await.ok().unwrap();
        let response_body = response_body.as_object().unwrap();

        let token = response_body.get("access_token").unwrap().as_str().unwrap();

        let user_response = client
            .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
            .header("Authorization", "Bearer ".to_owned() + token)
            .send()
            .await
            .ok();
        let user_response = user_response.unwrap();

        if user_response.status() == StatusCode::OK {
            let mut google_user = user_response.json::<GoogleUser>().await.ok().unwrap();

            let response: (Status, Option<User>) = create_user(state, &google_user).await;

            if response.1.is_none() {
                return Response::unauthorized(None);
            }

            let user = response.1.unwrap();

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
            cookie.set_domain(if domain == "" {
                String::from("localhost")
            } else {
                domain
            });
            cookies.add(cookie);

            if response.0 == Status::Created {
                return Response::created(String::from("User Created Successfully"));
            } else {
                return Response::ok(String::from("User Logged In Successfully"));
            }
        }
    }
    Response::unauthorized(None)
}

pub async fn get_users(state: &State<AppState>) -> (Status, String) {
    let mut users: Vec<UserResponse> = Vec::new();
    // let collection: Collection<User> = get_collection(state, "user").await;

    let collection_result: Option<Collection<User>> = tools::get_collection(state, "user").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<User> = collection_result.unwrap();

    let result = collection.find(doc! {}).await;

    let cursor = match result {
        Ok(cursor) => cursor,
        Err(_) => return Response::no_content(None),
    };

    cursor
        .try_collect()
        .await
        .unwrap_or(vec![])
        .iter()
        .for_each(|user| {
            users.push(UserResponse::copy(user));
        });

    Response::ok(serde_json::to_string(&users).unwrap())
}

pub async fn get_user_by_id(state: &State<AppState>, id: String) -> Option<UserResponse> {
    if !ObjectId::parse_str(&id).is_ok() {
        return None;
    }

    let user_id = ObjectId::parse_str(&id).ok().unwrap_or_default();

    let collection_result: Option<Collection<User>> = tools::get_collection(state, "user").await;
    if collection_result.is_none() {
        return None;
    }
    let collection: Collection<User> = collection_result.unwrap();

    let user = collection
        .find_one(doc! {"_id": user_id})
        .await
        .ok()
        .unwrap();

    if user.is_none() {
        return None;
    }

    user.map(|u| UserResponse::copy(&u))
}

// pub async fn get_users_by_ids(state: &State<AppState>, users: Vec<String>) -> Vec<UserResponse> -> {

// }

async fn create_user(state: &State<AppState>, google_user: &GoogleUser) -> (Status, Option<User>) {
    let mut user_id: String = String::from("0");

    let collection_result: Option<Collection<User>> = tools::get_collection(state, "user").await;
    if collection_result.is_none() {
        return (Status::InternalServerError, None);
    }
    let collection: Collection<User> = collection_result.unwrap();

    let existing_user: Option<User> = collection
        .find_one(doc! {"email": &google_user.email })
        .await
        .ok()
        .unwrap();

    if existing_user.is_some() {
        let existing_user = &existing_user.unwrap();
        return (Status::Ok, Some((*existing_user).clone()));
    }

    let user = User::new(&google_user, String::from("user"));

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(&user).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        user_id = inserted_id.to_hex();
    }

    if user_id != "0" {
        return (Status::Created, Some(user));
    } else {
        return (Status::Unauthorized, None);
    }
}
