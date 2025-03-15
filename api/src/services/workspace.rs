use mongodb::{bson::{doc, oid::ObjectId}, Collection};
use rocket::{futures::TryStreamExt, http::Status, serde::json::Json, State};

use crate::{models::workspace::{Workspace, WorkspaceRequest, WorkspaceResponse}, AppState};

pub async fn get_workspaces(state: &State<AppState>) -> (Status, Json<Vec<WorkspaceResponse>>) {
    let mut workspaces: Vec<WorkspaceResponse> = Vec::new();
    let collection: Collection<Workspace> = get_collection(state, "workspace").await;

    let result = collection.find(doc! {}).await;
    let cursor = match result {
        Ok(cursor) => cursor,
        Err(_) => return (Status::BadRequest, Json(vec![])),
    };

    cursor
        .try_collect()
        .await
        .unwrap_or(vec![])
        .iter()
        .for_each(|workspace| {
            workspaces.push(WorkspaceResponse::new(
                workspace._id.to_hex(),
                workspace.owner.to_string(),
                workspace.name.to_string(),
                workspace.deleted.to_string().parse().unwrap(),
                workspace.is_active.to_string().parse().unwrap(),
                workspace.created.clone()
            ));
        });

    (Status::Ok, Json(workspaces))
}

pub async fn create_workspace(state: &State<AppState>, workspaceBody: Json<WorkspaceRequest>, owner: String) -> (Status, Json<String>) {
    let mut workspace_id = String::from("0");
    let collection = get_collection(state, "workspace").await;
    let mut workspace = Workspace::try_from(workspaceBody.into_inner()).unwrap();

    workspace.owner = ObjectId::parse_str(owner).ok().unwrap_or_default();
    
    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(workspace).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        workspace_id = inserted_id.to_hex();
    }

    (Status::Ok, Json(String::from(workspace_id)))
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<Workspace> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("tasks");
    db.collection::<Workspace>(collection)
}