use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use rocket::{State, futures::TryStreamExt, http::Status, serde::json::Json};

use crate::{
    models::{response::Response, tag::{Tag, TagRequest, TagResponse, Visibility}}, utils::request_guard::HeaderGuard, AppState
};

pub async fn get_tags(_guard: HeaderGuard, state: &State<AppState>, workspace_id: String) -> (Status, String) {
    let user_id = ObjectId::parse_str(_guard._get_id()).ok().unwrap();
    let mut tags: Vec<TagResponse> = Vec::new();

    if !ObjectId::parse_str(&workspace_id).is_ok() {
        return Response::bad_request(None);
    }

    let workspace_id = ObjectId::parse_str(workspace_id).ok().unwrap_or_default();

    let collection: Collection<Tag> = get_collection(state, "tag").await;

    let result = collection.find(doc! {"workspace": workspace_id, "deleted": false}).await;
    let cursor = match result {
        Ok(cursor) => cursor,
        Err(_) => return Response::bad_request(None),
    };

    cursor
        .try_collect()
        .await
        .unwrap_or(vec![])
        .iter()
        .filter(|tag| tag.visibility == Visibility::PUBLIC || (tag.visibility == Visibility::PRIVATE && tag.created_by == user_id))
        .for_each(|tag| {
            tags.push(TagResponse::copy(tag));
        });

    Response::ok(serde_json::to_string(&tags).unwrap())
}

pub async fn create_tag(
    state: &State<AppState>,
    tag_body: Json<TagRequest>,
    owner: String,
) -> (Status, String) {
    let mut tag_id = String::from("0");
    let collection = get_collection(state, "tag").await;
    let tag_result = Tag::try_from(tag_body.into_inner());
    
    if let Err(err) = tag_result {
        let error: String = Some(err).unwrap().to_string();
        return Response::internal_server_error(Some(error));
    }

    let mut tag = tag_result.unwrap();

    tag.created_by = ObjectId::parse_str(owner).ok().unwrap();

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(tag).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        tag_id = inserted_id.to_hex();
    }
    
    Response::ok(tag_id)
}

pub async fn delete_tag(state: &State<AppState>, tag_id: &str) -> (Status, String) {
    if !ObjectId::parse_str(&tag_id).is_ok() {
        return Response::bad_request(None);
    }

    let tag_id = ObjectId::parse_str(tag_id).ok().unwrap_or_default();

    let collection = get_collection(state, "tag").await;
    let existing_tag_result = collection.find_one(doc! {"_id": tag_id }).await;

    if existing_tag_result.ok().unwrap().is_none() {
        return Response::not_found(None);
    }

    collection
        .update_one(
            doc! {"_id": tag_id},
            doc! { "$set": doc! {"deleted": true} },
        )
        .await
        .ok()
        .unwrap();

    Response::ok(stringify!(true).to_string())
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<Tag> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("tasks");
    db.collection::<Tag>(collection)
}
