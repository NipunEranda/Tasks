use rocket::{State, delete, get, http::Status, post, serde::json::Json};

use crate::{
    AppState,
    models::tag::{TagRequest, TagResponse},
    services,
    utils::request_guard::HeaderGuard,
};

#[get("/tag/<workspace_id>")]
pub async fn get_tags(
    _guard: HeaderGuard,
    state: &State<AppState>,
    workspace_id: &str,
) -> (Status, Json<Vec<TagResponse>>) {
    services::tag::get_tags(state, String::from(workspace_id)).await
}

#[post("/tag", data = "<tag>")]
pub async fn create_tag(
    _guard: HeaderGuard,
    state: &State<AppState>,
    tag: Json<TagRequest>,
) -> (Status, Json<String>) {
    services::tag::create_tag(state, tag, _guard._get_id()).await
}

#[delete("/tag/<id>")]
pub async fn delete_tag(
    _guard: HeaderGuard,
    state: &State<AppState>,
    id: &str,
) -> (Status, Json<bool>) {
    services::tag::delete_tag(state, id).await
}
