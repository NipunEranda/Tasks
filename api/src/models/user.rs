// use std::time::SystemTime;

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
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

// impl User {
//     pub fn new(name: String, email: String, picture: String) -> Self {
//         let chrono_datetime: SystemTime = chrono::Utc::now().into();
//         User {
//             _id: ObjectId::new(),
//             name,
//             email,
//             picture,
//             created: DateTime::from(chrono_datetime),
//             is_active: true,
//         }
//     }
// }