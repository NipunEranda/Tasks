use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use rocket::{State, futures::TryStreamExt, http::Status, serde::json::Json};

use crate::{
    models::tag::{Tag, TagRequest, TagResponse, Visibility}, utils::request_guard::HeaderGuard, AppState
};

pub async fn get_tags(_guard: HeaderGuard, state: &State<AppState>, workspace_id: String) -> (Status, Json<Vec<TagResponse>>) {
    let user_id = ObjectId::parse_str(_guard._get_id()).ok().unwrap();
    let mut tags: Vec<TagResponse> = Vec::new();

    if !ObjectId::parse_str(&workspace_id).is_ok() {
        return (Status::BadRequest, Json(vec![]));
    }

    let workspace_id = ObjectId::parse_str(workspace_id).ok().unwrap_or_default();

    let collection: Collection<Tag> = get_collection(state, "tag").await;

    let result = collection.find(doc! {"workspace": workspace_id, "deleted": false}).await;
    let cursor = match result {
        Ok(cursor) => cursor,
        Err(_) => return (Status::BadRequest, Json(vec![])),
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

    (Status::Ok, Json(tags))
}

pub async fn create_tag(
    state: &State<AppState>,
    tag_body: Json<TagRequest>,
    owner: String,
) -> (Status, Json<String>) {
    let mut tag_id = String::from("0");
    let collection = get_collection(state, "tag").await;
    let tag_result = Tag::try_from(tag_body.into_inner());
    
    if let Err(err) = tag_result {
        return (Status::InternalServerError, Json(err.to_string()));
    }

    let mut tag = tag_result.unwrap();

    tag.created_by = ObjectId::parse_str(owner).ok().unwrap();

    println!("{:?}", tag);

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(tag).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        tag_id = inserted_id.to_hex();
    }

    (Status::Ok, Json(String::from(tag_id)))
}

pub async fn delete_tag(state: &State<AppState>, tag_id: &str) -> (Status, Json<bool>) {
    if !ObjectId::parse_str(&tag_id).is_ok() {
        return (Status::BadRequest, Json(false));
    }

    let tag_id = ObjectId::parse_str(tag_id).ok().unwrap_or_default();

    let collection = get_collection(state, "tag").await;
    let existing_tag_result = collection.find_one(doc! {"_id": tag_id }).await;

    if existing_tag_result.ok().unwrap().is_none() {
        return (Status::NotFound, Json(false));
    }

    collection
        .update_one(
            doc! {"_id": tag_id},
            doc! { "$set": doc! {"deleted": true} },
        )
        .await
        .ok()
        .unwrap();

    (Status::Ok, Json(true))
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<Tag> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("tasks");
    db.collection::<Tag>(collection)
}
