use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    sub: String,
    name: String,
    username: String,
    picture: String,
}

impl Claims {
    pub fn new(sub: String, name: String, username: String, picture: String) -> Self {
        Claims {
            exp: Utc::now()
                .checked_add_signed(chrono::Duration::seconds(86400))
                .expect("Valid timestamp")
                .timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            sub,
            name,
            username,
            picture
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
        return String::from(&self.sub);
    }
}