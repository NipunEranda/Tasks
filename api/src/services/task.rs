use mongodb::Collection;
use rocket::{State, http::Status, serde::json::Json};

use crate::{models::{response::Response, task::{Task, TaskRequest}}, utils::request_guard::HeaderGuard, AppState};

pub async fn create_task_template(
    _guard: HeaderGuard,
    state: &State<AppState>,
    task_body: Json<TaskRequest>,
) -> (Status, String) {
    let mut task_id = String::from("0");
    let collection: Collection<Task> = get_collection(state, "templates").await;
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

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<Task> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("tasks");
    db.collection::<Task>(collection)
}
