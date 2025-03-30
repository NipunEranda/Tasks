use mongodb::Collection;
use rocket::State;

use crate::AppState;

pub async fn get_collection<T: std::marker::Send + Sync>(state: &State<AppState>, collection: &str) -> Option<Collection<T>> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("tasks");
    match collection {
        "templates" => 
        Some(db.collection::<T>(collection)),
        "tasks" =>
        Some(db.collection::<T>(collection)),
        "tag" =>
        Some(db.collection::<T>(collection)),
        "user" =>
        Some(db.collection::<T>(collection)),
        "workspace" =>
        Some(db.collection::<T>(collection)),
        _ => None
    }
}