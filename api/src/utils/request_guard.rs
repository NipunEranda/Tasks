use std::env;

use jsonwebtoken::{DecodingKey, Validation, decode};
use rocket::{Request, http::Status, outcome, request::FromRequest};
use serde::{Deserialize, Serialize};

use crate::models::claims::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderGuard {
    id: String,
    username: String,
    name: String,
    picture: String,
    authorized: bool,
}

impl HeaderGuard {
    pub fn _get_id(&self) -> String {
        return String::from(&self.id);
    }

    pub fn _get_username(&self) -> String {
        return String::from(&self.username);
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HeaderGuard {
    type Error = std::io::Error;

    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let mut token: String = "".to_string();
        // let cookie_token = req.cookies().get("tak").unwrap().value();
        if let Some(ct) = req.cookies().get("tak"){
            token = ct.value().to_string();
        }

        if let Some(bearer_token) = req.headers().get_one("Authorization") {
            let token_split: Vec<&str> = bearer_token.split(" ").collect();
            if !token_split.get(1).is_none() {
                token = token_split.get(1).unwrap().to_string();
            }
        }

        if token != "" {
            let decoded_value = decode::<Claims>(
                token.as_str(),
                &DecodingKey::from_secret(env::var("JWT_KEY").ok().unwrap().as_bytes()),
                &Validation::default(),
            );

            if decoded_value.is_ok() {
                let claims_data = decoded_value.ok().unwrap().claims;
                return outcome::Outcome::Success(HeaderGuard {
                    id: claims_data._get_id(),
                    username: claims_data._get_username(),
                    name: claims_data._get_name(),
                    picture: claims_data._get_picture(),
                    authorized: true,
                });
            }
        }

        return outcome::Outcome::Error((
            (Status::Forbidden),
            std::io::Error::new(std::io::ErrorKind::Other, "Unauthorized"),
        ));
    }
}
