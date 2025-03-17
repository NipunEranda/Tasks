use rocket::{delete, get, http::Status, post, put, serde::json::Json, State};

use crate::{models::workspace::{WorkspaceRequest, WorkspaceResponse}, services, utils::request_guard::HeaderGuard, AppState};

#[get("/workspace")]
pub async fn get_workspaces(
    _guard: HeaderGuard,
    state: &State<AppState>,
) -> (Status, Json<Vec<WorkspaceResponse>>) {
    services::workspace::get_workspaces(state).await
}

#[post("/workspace", data = "<workspace>")]
pub async fn create_workspace(_guard: HeaderGuard, state: &State<AppState>, workspace: Json<WorkspaceRequest>) -> (Status, Json<String>) {
    services::workspace::create_workspace(state, workspace, _guard._get_id()).await
}

#[put("/workspace/<id>", data = "<workspace>")]
pub async fn update_workspace(_guard: HeaderGuard, state: &State<AppState>, workspace: Json<WorkspaceRequest>, id: &str) -> (Status, Json<bool>) {
    services::workspace::update_workspace(state, id, workspace).await
}

#[delete("/workspace/<id>")]
pub async fn delete_workspace(_guard: HeaderGuard, state: &State<AppState>, id: &str) -> (Status, Json<bool>) {
    services::workspace::delete_workspace(state, id).await
}