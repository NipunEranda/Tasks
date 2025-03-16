// use std::time::SystemTime;

use std::time::SystemTime;

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub picture: String,
    pub role: String,
    pub created: DateTime,
    pub is_active: bool,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub picture: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleResponse {
    pub sid: String,
    pub name: String,
    pub email: String,
    pub picture: String
}

impl User {
    pub fn new(google_user: &GoogleUser, role: String) -> Self {
        let chrono_datetime: SystemTime = chrono::Utc::now().into();
        User {
            _id: ObjectId::new(),
            name: String::from(&google_user.name),
            email: String::from(&google_user.email),
            picture:String::from(& google_user.picture),
            role,
            created: DateTime::from(chrono_datetime),
            is_active: true,
        }
    }
}

impl GoogleUser {
    pub fn new(id: String, name: String, email: String, picture: String) -> Self {
        GoogleUser {
            id,
            name,
            email,
            picture,
        }
    }
}