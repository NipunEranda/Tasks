use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::{models::workspace::{WorkspaceRequest, WorkspaceResponse}, services, utils::request_guard::HeaderGuard, AppState};

#[get("/workspace")]
pub async fn get_workspaces(
    _guard: HeaderGuard,
    state: &State<AppState>,
) -> (Status, Json<Vec<WorkspaceResponse>>) {
    services::workspace::get_workspaces(state).await
}

#[post("/workspace", format = "json", data = "<workspace>")]
pub async fn create_workspace(_guard: HeaderGuard, state: &State<AppState>, workspace: Json<WorkspaceRequest>) -> (Status, Json<String>) {
    services::workspace::create_workspace(state, workspace, _guard._get_id()).await
}