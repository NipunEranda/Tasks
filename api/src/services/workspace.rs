use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use rocket::{State, futures::TryStreamExt, http::Status, serde::json::Json};

use crate::{
    AppState,
    models::workspace::{Workspace, WorkspaceRequest, WorkspaceResponse},
};

pub async fn get_workspaces(state: &State<AppState>) -> (Status, Json<Vec<WorkspaceResponse>>) {
    let mut workspaces: Vec<WorkspaceResponse> = Vec::new();
    let collection: Collection<Workspace> = get_collection(state, "workspace").await;

    let result = collection.find(doc! {"deleted": false, "is_active": true}).await;
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
                workspace.created.to_string(),
            ));
        });

    (Status::Ok, Json(workspaces))
}

pub async fn create_workspace(
    state: &State<AppState>,
    workspace_body: Json<WorkspaceRequest>,
    owner: String,
) -> (Status, Json<String>) {
    let mut workspace_id = String::from("0");
    let collection = get_collection(state, "workspace").await;
    let mut workspace = Workspace::try_from(workspace_body.into_inner()).unwrap();

    println!("{:?}", owner);

    workspace.owner = ObjectId::parse_str(owner).ok().unwrap_or_default();

    println!("{:?}", workspace.owner);

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(workspace).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        workspace_id = inserted_id.to_hex();
    }

    (Status::Ok, Json(String::from(workspace_id)))
}

pub async fn update_workspace(
    state: &State<AppState>,
    workspace_id: &str,
    workspace_body: Json<WorkspaceRequest>,
) -> (Status, Json<bool>) {
    if !ObjectId::parse_str(&workspace_id).is_ok() {
        return (Status::BadRequest, Json(false));
    }

    let workspace_id = ObjectId::parse_str(workspace_id).ok().unwrap_or_default();

    let collection = get_collection(state, "workspace").await;
    let existing_workspace_result = collection
        .find_one(doc! {"_id": workspace_id })
        .await;

    if existing_workspace_result.ok().unwrap().is_none() {
        return (Status::NotFound, Json(false));
    }

    let workspace = Workspace::try_from(workspace_body.into_inner()).unwrap();

    collection
        .update_one(
            doc! {"_id": workspace_id},
            doc! { "$set": doc! {"name": workspace.name} },
        )
        .await
        .ok()
        .unwrap();

    (Status::Ok, Json(true))
}

pub async fn delete_workspace(state: &State<AppState>, workspace_id: &str) -> (Status, Json<bool>) {
    if !ObjectId::parse_str(&workspace_id).is_ok() {
        return (Status::BadRequest, Json(false));
    }

    let workspace_id = ObjectId::parse_str(workspace_id).ok().unwrap_or_default();

    let collection = get_collection(state, "workspace").await;
    let existing_workspace_result = collection
        .find_one(doc! {"_id": workspace_id })
        .await;

    if existing_workspace_result.ok().unwrap().is_none() {
        return (Status::NotFound, Json(false));
    }

    collection
        .update_one(
            doc! {"_id": workspace_id},
            doc! { "$set": doc! {"deleted": true, "is_active": false} },
        )
        .await
        .ok()
        .unwrap();

    (Status::Ok, Json(true))
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<Workspace> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("tasks");
    db.collection::<Workspace>(collection)
}
