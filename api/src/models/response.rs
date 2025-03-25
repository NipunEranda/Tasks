use rocket::http::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: String,
}

impl Response {
    pub fn ok(data: String) -> (Status, String) {
        (Status::Ok, data)
    }
    pub fn created(data: String) -> (Status, String) {
        (Status::Created, data)
    }
    pub fn accepted(custom_message: Option<String>) -> (Status, String) {
        match custom_message {
            Some(message) => (Status::Accepted, message),
            None => (Status::Accepted, String::from("Accepted")),
        }
    }
    pub fn no_content(custom_message: Option<String>) -> (Status, String) {
        match custom_message {
            Some(message) => (Status::Accepted, message),
            None => (Status::Accepted,String::from("No content")),
        }
    }
    pub fn not_found(custom_message: Option<String>) -> (Status, String) {
        match custom_message {
            Some(message) => (Status::Accepted, message),
            None => (Status::Accepted, String::from("Data not found")),
        }
    }
    pub fn bad_request(custom_message: Option<String>) -> (Status, String) {
        match custom_message {
            Some(message) => (Status::Accepted, message),
            None => (Status::Accepted, String::from("Bad request")),
        }
    }
    pub fn internal_server_error(custom_message: Option<String>) -> (Status, String) {
        match custom_message {
            Some(message) => (Status::Accepted, message),
            None => (Status::Accepted, String::from("Internal server error")),
        }
    }
    pub fn forbidden(custom_message: Option<String>) -> (Status, String) {
        match custom_message {
            Some(message) => (Status::Accepted, message),
            None => (Status::Accepted, String::from("Forbidden to the operations")),
        }
    }
    pub fn unsupported_media_type(custom_message: Option<String>) -> (Status, String) {
        match custom_message {
            Some(message) => (Status::Accepted, message),
            None => (Status::Accepted, String::from("Unsupported media type")),
        }
    }
    pub fn unauthorized(custom_message: Option<String>) -> (Status, String) {
        match custom_message {
            Some(message) => (Status::Accepted, message),
            None => (Status::Accepted, String::from("Unauthorized to the operations")),
        }
    }
    pub fn not_modified(custom_message: Option<String>) -> (Status, String) {
        match custom_message {
            Some(message) => (Status::Accepted, message),
            None => (Status::Accepted, String::from("Data not modified")),
        }
    }
}