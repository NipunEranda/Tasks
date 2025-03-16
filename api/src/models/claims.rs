use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::user::GoogleUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    id: String,
    name: String,
    username: String,
    picture: String,
    role: String,
}

impl Claims {
    pub fn new(google_user: GoogleUser) -> Self {
        Claims {
            exp: Utc::now()
                .checked_add_signed(chrono::Duration::seconds(86400))
                .expect("Valid timestamp")
                .timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            id: google_user.id,
            name: google_user.name,
            username: google_user.email,
            picture: google_user.picture,
            role: String::from("user"),
        }
    }

    pub fn _get_name(&self) -> String {
        return String::from(&self.name);
    }

    pub fn _get_username(&self) -> String {
        return String::from(&self.username);
    }

    pub fn _get_picture(&self) -> String {
        return String::from(&self.picture);
    }

    pub fn _get_id(&self) -> String {
        return String::from(&self.id);
    }

    pub fn _get_role(&self) -> String {
        return String::from(&self.role);
    }

    pub fn _set_role(&mut self, role: String) {
        self.role = role;
    }
}