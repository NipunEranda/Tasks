use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::{
    models::task::TaskRequest, services, utils::request_guard::HeaderGuard, AppState
};

#[get("/task/template/<workspace_id>")]
pub async fn get_template(
    _guard: HeaderGuard,
    state: &State<AppState>,
    workspace_id: &str,
) -> (Status, String) {
    services::task::get_task_template(_guard, state, workspace_id).await
}

#[post("/task/template", data = "<template>")]
pub async fn create_template(
    _guard: HeaderGuard,
    state: &State<AppState>,
    template: Json<TaskRequest>,
) -> (Status, String) {
    services::task::create_task_template(_guard, state, template).await
}