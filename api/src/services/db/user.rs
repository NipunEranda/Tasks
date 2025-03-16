use mongodb::{Collection, bson::doc};
use rocket::State;

use crate::{
    AppState,
    models::user::{GoogleUser, User},
};

pub async fn create_user(state: &State<AppState>, google_user: &GoogleUser) -> String {
    let mut user_id: String = String::from("0");
    let collection: Collection<User> = get_collection(state, "user").await;

    let user = User::new(google_user);

    let existing_user: Option<User> = collection
        .find_one(doc! {"email": &user.email })
        .await
        .ok()
        .unwrap();

    if existing_user.is_some() {
        let existing_user = &existing_user.unwrap();
        return existing_user._id.to_hex();
    }

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(user).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        user_id = inserted_id.to_hex();
    }

    user_id
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<User> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("tasks");
    db.collection::<User>(collection)
}
