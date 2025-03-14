use std::time::SystemTime;

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub picture: String,
    pub created: DateTime,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleAuth {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

impl User {
    pub fn new(name: String, email: String, picture: String) -> Self {
        let chrono_datetime: SystemTime = chrono::Utc::now().into();
        User {
            _id: ObjectId::new(),
            name,
            email,
            picture,
            created: DateTime::from(chrono_datetime),
            is_active: true,
        }
    }
}

impl GoogleAuth {
    pub fn new(
        access_token: String,
        refresh_token: String,
        expires_in: String,
        scope: String,
        token_type: String,
        id_token: String,
    ) -> Self {
        GoogleAuth {
            access_token,
            refresh_token,
            expires_in,
            scope,
            token_type,
            id_token,
        }
    }
}

impl TryFrom<GoogleAuth> for LoginResponse {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: GoogleAuth) -> Result<Self, Self::Error> {
        Ok(Self {
            access_token: item.access_token,
            refresh_token: item.refresh_token,
            expires_in: item.expires_in,
            scope: item.scope,
            token_type: item.token_type,
            id_token: item.id_token,
        })
    }
}
