use rocket::{State, get, http::Status, serde::json::Json};

use crate::{AppState, models::workspace::WorkspaceResponse, utils::request_guard::HeaderGuard};

#[get("/workspace")]
pub async fn index(
    _guard: HeaderGuard,
    state: &State<AppState>,
) -> (Status, Json<Vec<WorkspaceResponse>>) {
    println!("{:?}", _guard);
    (Status::BadRequest, Json(vec![]))
}
