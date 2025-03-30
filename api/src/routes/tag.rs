use rocket::{State, delete, get, http::Status, post, serde::json::Json};

use crate::{
    AppState,
    models::tag::TagRequest,
    services,
    utils::request_guard::HeaderGuard,
};

#[get("/tag/<workspace_id>")]
pub async fn get_tags(
    _guard: HeaderGuard,
    state: &State<AppState>,
    workspace_id: &str,
) -> (Status, String) {
    services::tag::get_tags(_guard, state, String::from(workspace_id)).await
}

#[post("/tag", data = "<tag>")]
pub async fn create_tag(
    _guard: HeaderGuard,
    state: &State<AppState>,
    tag: Json<TagRequest>,
) -> (Status, String) {
    services::tag::create_tag(state, tag, _guard._get_id().unwrap().to_hex()).await
}

#[delete("/tag/<id>")]
pub async fn delete_tag(
    _guard: HeaderGuard,
    state: &State<AppState>,
    id: &str,
) -> (Status, String) {
    services::tag::delete_tag(state, id).await
}
