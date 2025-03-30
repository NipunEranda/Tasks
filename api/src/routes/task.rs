use rocket::{State, http::Status, post, serde::json::Json};

use crate::{
    models::task::TaskRequest, services, utils::request_guard::HeaderGuard, AppState
};

#[post("/task/template", data = "<template>")]
pub async fn create_template(
    _guard: HeaderGuard,
    state: &State<AppState>,
    template: Json<TaskRequest>,
) -> (Status, String) {
    services::task::create_task_template(_guard, state, template).await
}