use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use rocket::{State, futures::TryStreamExt, http::Status, serde::json::Json};

use crate::{
    models::{response::Response, tag::{Tag, TagRequest, TagResponse}}, utils::{request_guard::HeaderGuard, tools}, AppState
};

pub async fn get_tags(_guard: HeaderGuard, state: &State<AppState>, workspace_id: String) -> (Status, String) {
    let user_id = ObjectId::parse_str(_guard._get_id().unwrap().to_hex()).ok().unwrap();
    let mut tags: Vec<TagResponse> = Vec::new();

    if !ObjectId::parse_str(&workspace_id).is_ok() {
        return Response::bad_request(None);
    }

    let workspace_id = ObjectId::parse_str(workspace_id).ok().unwrap_or_default();

    let collection_result: Option<Collection<Tag>> = tools::get_collection(state, "tag").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<Tag> = collection_result.unwrap();

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
        .filter(|tag| !tag.is_private || (tag.is_private && tag.created_by == user_id))
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

    let collection_result: Option<Collection<Tag>> = tools::get_collection(state, "tag").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<Tag> = collection_result.unwrap();

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

    let collection_result: Option<Collection<Tag>> = tools::get_collection(state, "tag").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<Tag> = collection_result.unwrap();

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