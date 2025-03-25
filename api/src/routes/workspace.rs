use std::env;

use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};

use crate::{
    AppState, models::workspace::WorkspaceRequest, services, utils::request_guard::HeaderGuard,
};

#[get("/workspace/test")]
pub async fn test() -> (Status, Json<String>) {
    (Status::Ok, Json(env::var("TEST_ENV").ok().unwrap()))
}

#[get("/workspace")]
pub async fn get_workspaces(_guard: HeaderGuard, state: &State<AppState>) -> (Status, String) {
    services::workspace::get_workspaces(_guard, state).await
}

#[get("/workspace/<id>/team")]
pub async fn get_workspace_team(_guard: HeaderGuard, state: &State<AppState>, id: &str) -> (Status, String) {
    services::workspace::get_workspace_team(_guard, state, id).await
}

#[post("/workspace", data = "<workspace>")]
pub async fn create_workspace(
    _guard: HeaderGuard,
    state: &State<AppState>,
    workspace: Json<WorkspaceRequest>,
) -> (Status, String) {
    services::workspace::create_workspace(state, workspace, _guard._get_id()).await
}

#[put("/workspace/<id>", data = "<workspace>")]
pub async fn update_workspace(
    _guard: HeaderGuard,
    state: &State<AppState>,
    workspace: Json<WorkspaceRequest>,
    id: &str,
) -> (Status, String) {
    services::workspace::update_workspace(state, id, workspace).await
}

#[delete("/workspace/<id>")]
pub async fn delete_workspace(
    _guard: HeaderGuard,
    state: &State<AppState>,
    id: &str,
) -> (Status, String) {
    services::workspace::delete_workspace(state, id).await
}
