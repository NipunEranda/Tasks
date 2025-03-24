use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: String,
}

impl Response {
    pub fn Ok(data: String) -> (Status, Json<Response>) {
        (Status::Ok, Json(Response { data }))
    }
    pub fn Created(data: String) -> (Status, Json<Response>) {
        (Status::Created, Json(Response { data }))
    }
    pub fn Accepted(data: String) -> (Status, Json<Response>) {
        (Status::Accepted, Json(Response { data }))
    }
    pub fn NoContent(data: String) -> (Status, Json<Response>) {
        (Status::NoContent, Json(Response { data }))
    }
    pub fn NotFound(data: String) -> (Status, Json<Response>) {
        (Status::NotFound, Json(Response { data }))
    }
    pub fn BadRequest(data: String) -> (Status, Json<Response>) {
        (Status::BadRequest, Json(Response { data }))
    }
    pub fn InternalServerError(data: String) -> (Status, Json<Response>) {
        (Status::InternalServerError, Json(Response { data }))
    }
    pub fn Forbidden(data: String) -> (Status, Json<Response>) {
        (Status::Forbidden, Json(Response { data }))
    }
    pub fn UnsupportedMediaTypecode415(data: String) -> (Status, Json<Response>) {
        (Status::UnsupportedMediaType, Json(Response { data }))
    }
    pub fn Unauthorized(data: String) -> (Status, Json<Response>) {
        (Status::Unauthorized, Json(Response { data }))
    }
    pub fn NotModified(data: String) -> (Status, Json<Response>) {
        (Status::NotModified, Json(Response { data }))
    }
}