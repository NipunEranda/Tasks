use mongodb::{bson::{doc, oid::ObjectId}, Collection};
use rocket::{State, futures::TryStreamExt, http::Status, serde::json::Json};

use crate::{
    AppState,
    models::{
        response::Response,
        task::{Task, TaskRequest, TaskResponse},
    },
    utils::{request_guard::HeaderGuard, tools},
};

pub async fn get_task_template(
    _guard: HeaderGuard,
    state: &State<AppState>,
    workspace_id: &str,
) -> (Status, String) {
    let mut templates: Vec<TaskResponse> = Vec::new();

    if !ObjectId::parse_str(&workspace_id).is_ok() {
        return Response::bad_request(None);
    }

    let workspace_id = ObjectId::parse_str(workspace_id).ok().unwrap_or_default();

    let collection_result: Option<Collection<Task>> =
        tools::get_collection(state, "templates").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<Task> = collection_result.unwrap();

    let result = collection
        .find(doc! {"deleted": false, "workspace": workspace_id})
        .await;
    let cursor = match result {
        Ok(cursor) => cursor,
        Err(_) => return Response::bad_request(None),
    };

    cursor
        .try_collect()
        .await
        .unwrap_or(vec![])
        .iter()
        .filter(|template: &&Task| !template.deleted)
        .for_each(|template| {
            templates.push(TaskResponse::copy(template));
        });

    Response::ok(serde_json::to_string(&templates).unwrap())
}

pub async fn create_task_template(
    _guard: HeaderGuard,
    state: &State<AppState>,
    task_body: Json<TaskRequest>,
) -> (Status, String) {
    let mut task_id = String::from("0");

    let collection_result: Option<Collection<Task>> =
        tools::get_collection(state, "templates").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<Task> = collection_result.unwrap();

    let task_request = Task::try_from(task_body.into_inner());

    if let Err(err) = task_request {
        let error: String = Some(err).unwrap().to_string();
        return Response::internal_server_error(Some(error));
    }

    let mut task = task_request.unwrap();

    task.created_by = _guard._get_id().unwrap();
    task.updated_by = _guard._get_id().unwrap();

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(task).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        task_id = inserted_id.to_hex();
    }

    Response::ok(task_id)
}
