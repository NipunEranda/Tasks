use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use rocket::{futures::{self, TryStreamExt}, http::Status, serde::json::Json, State};

use crate::{
    models::{
        response::Response, user::UserResponse, workspace::{Workspace, WorkspaceRequest, WorkspaceResponse}
    }, utils::{request_guard::HeaderGuard, tools}, AppState
};

use super::user::get_user_by_id;

pub async fn get_workspaces(_guard: HeaderGuard, state: &State<AppState>) -> (Status, String) {
    let user_id = ObjectId::parse_str(_guard._get_id().unwrap().to_hex()).ok().unwrap();
    let mut workspaces: Vec<WorkspaceResponse> = Vec::new();
    let collection_result: Option<Collection<Workspace>> = tools::get_collection(state, "workspace").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<Workspace> = collection_result.unwrap();

    let result = collection
        .find(doc! {"deleted": false, "is_active": true})
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
        .filter(|workspace| {
            (workspace.team.contains(&user_id) && workspace.is_private) || workspace.owner == user_id || !workspace.is_private
        })
        .for_each(|workspace| {
            workspaces.push(WorkspaceResponse::new(
                workspace._id.to_hex(),
                workspace.owner.to_string(),
                workspace.name.to_string(),
                workspace.is_private,
                workspace.team.clone(),
                workspace.deleted.to_string().parse().unwrap(),
                workspace.is_active.to_string().parse().unwrap(),
                workspace.created.to_string(),
            ));
        });

    Response::ok(serde_json::to_string(&workspaces).unwrap())
}

pub async fn get_workspace_team(
    _guard: HeaderGuard,
    state: &State<AppState>,
    id: &str,
) -> (Status, String) {
    let mut team: Vec<UserResponse> = vec![];
    if !ObjectId::parse_str(&id).is_ok() {
        return Response::bad_request(None);
    }

    let workspace = get_workspace(&state, &id).await;

    if workspace.is_none() {
        return Response::not_found(Some(String::from("Workspace Not Found")));
    }

    let workspace = workspace.unwrap();
    
    let user_futures = workspace.team.iter().map(|user_id| {
        get_user_by_id(state, user_id.to_hex())
    });

    let users = futures::future::join_all(user_futures).await;

    for user in users {
        let user_response = get_user_by_id(state, user.unwrap().id).await;
        if user_response.is_none() {
            continue;
        }

        team.push(user_response.unwrap());
    }

    Response::ok(serde_json::to_string(&team).unwrap())
}

pub async fn create_workspace(
    state: &State<AppState>,
    workspace_body: Json<WorkspaceRequest>,
    owner: String,
) -> (Status, String) {
    let mut workspace_id = String::from("0");

    let collection_result: Option<Collection<Workspace>> = tools::get_collection(state, "workspace").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<Workspace> = collection_result.unwrap();

    let mut workspace = Workspace::try_from(workspace_body.into_inner()).unwrap();

    workspace.owner = ObjectId::parse_str(owner).ok().unwrap_or_default();

    workspace.team.push(workspace.owner);

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(workspace).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        workspace_id = inserted_id.to_hex();
    }

    Response::ok(serde_json::to_string(&workspace_id).unwrap())
}

pub async fn update_workspace(
    state: &State<AppState>,
    workspace_id: &str,
    workspace_body: Json<WorkspaceRequest>,
) -> (Status, String) {
    if !ObjectId::parse_str(&workspace_id).is_ok() {
        return Response::bad_request(None);
    }

    let workspace_id = ObjectId::parse_str(workspace_id).ok().unwrap_or_default();

    let collection_result: Option<Collection<Workspace>> = tools::get_collection(state, "workspace").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<Workspace> = collection_result.unwrap();

    let existing_workspace_result = collection.find_one(doc! {"_id": workspace_id }).await;

    if existing_workspace_result.ok().unwrap().is_none() {
        return Response::not_found(None);
    }

    let workspace = Workspace::try_from(workspace_body.into_inner()).unwrap();

    collection
        .update_one(
            doc! {"_id": workspace_id},
            doc! { "$set": doc! {"name": workspace.name, "team": workspace.team} },
        )
        .await
        .ok()
        .unwrap();

    Response::ok(stringify!(true).to_string())
}

pub async fn delete_workspace(state: &State<AppState>, workspace_id: &str) -> (Status, String) {
    if !ObjectId::parse_str(&workspace_id).is_ok() {
        return Response::bad_request(None);
    }

    let workspace_id = ObjectId::parse_str(workspace_id).ok().unwrap_or_default();

    let collection_result: Option<Collection<Workspace>> = tools::get_collection(state, "workspace").await;
    if collection_result.is_none() {
        return Response::internal_server_error(None);
    }
    let collection: Collection<Workspace> = collection_result.unwrap();

    let existing_workspace_result = collection.find_one(doc! {"_id": workspace_id }).await;

    if existing_workspace_result.ok().unwrap().is_none() {
        return Response::not_found(None);
    }

    collection
        .update_one(
            doc! {"_id": workspace_id},
            doc! { "$set": doc! {"deleted": true, "is_active": false} },
        )
        .await
        .ok()
        .unwrap();

    return Response::ok(stringify!(true).to_string());
}

async fn get_workspace(state: &State<AppState>, id: &str) -> Option<Workspace> {
    let workspace_id = ObjectId::parse_str(id).ok().unwrap_or_default();
    
    let collection_result: Option<Collection<Workspace>> = tools::get_collection(state, "workspace").await;
    if collection_result.is_none() {
        return None;
    }
    let collection: Collection<Workspace> = collection_result.unwrap();

    let workspace_result = collection.find_one(doc! {"_id": workspace_id }).await;

    return workspace_result.ok().unwrap();
}