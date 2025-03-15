use std::env;

use jsonwebtoken::{DecodingKey, Validation, decode};
use rocket::{Request, http::Status, outcome, request::FromRequest};

use crate::models::claims::Claims;

#[derive(Debug)]
pub struct HeaderGuard {
    _id: String,
    _username: String,
    _name: String,
    _picture: String,
    _authorized: bool,
}

impl HeaderGuard {
    pub fn _get_id(&self) -> String {
        return String::from(&self._id);
    }

    pub fn _get_username(&self) -> String {
        return String::from(&self._username);
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HeaderGuard {
    type Error = std::io::Error;

    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        if let Some(bearer_token) = req.headers().get_one("Authorization") {
            let token_split: Vec<&str> = bearer_token.split(" ").collect();

            if !token_split.get(1).is_none() {
                let decoded_value = decode::<Claims>(
                    token_split.get(1).unwrap(),
                    &DecodingKey::from_secret(env::var("JWT_KEY").ok().unwrap().as_bytes()),
                    &Validation::default(),
                );

                if decoded_value.is_ok() {
                    let claims_data = decoded_value.ok().unwrap().claims;
                    return outcome::Outcome::Success(HeaderGuard {
                        _id: claims_data._get_id(),
                        _username: claims_data._get_username(),
                        _name: claims_data._get_name(),
                        _picture: claims_data._get_picture(),
                        _authorized: true,
                    });
                }
            }
        }

        return outcome::Outcome::Error((
            (Status::Forbidden),
            std::io::Error::new(std::io::ErrorKind::Other, "Unauthorized"),
        ));
    }
}
