use rocket::{delete, get, http::Status, post, serde::json::Json, State};

use crate::{models::tag::{TagRequest, TagResponse}, services, utils::request_guard::HeaderGuard, AppState};

#[get("/tag")]
pub async fn get_tags(
    _guard: HeaderGuard,
    state: &State<AppState>,
) -> (Status, Json<Vec<TagResponse>>) {
    services::tag::get_tags(state).await
}

#[post("/tag", data = "<tag>")]
pub async fn create_tag(_guard: HeaderGuard, state: &State<AppState>, tag: Json<TagRequest>) -> (Status, Json<String>) {
    println!("{:?}", _guard);
    services::tag::create_tag(state, tag, _guard._get_id()).await
}

#[delete("/tag/<id>")]
pub async fn delete_tag(_guard: HeaderGuard, state: &State<AppState>, id: &str) -> (Status, Json<bool>) {
    services::tag::delete_tag(state, id).await
}