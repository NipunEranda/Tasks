use mongodb::{Collection, bson::doc};
use rocket::{State, futures::TryStreamExt};

use crate::{
    AppState,
    models::user::{GoogleUser, User, UserResponse},
};

pub async fn create_user(state: &State<AppState>, google_user: &GoogleUser) -> User {
    // let mut user_id: String = String::from("0");
    let collection: Collection<User> = get_collection(state, "user").await;

    let user = User::new(google_user, String::from("user"));

    let existing_user: Option<User> = collection
        .find_one(doc! {"email": &user.email })
        .await
        .ok()
        .unwrap();

    if existing_user.is_some() {
        let existing_user = &existing_user.unwrap();
        return User::new(
            &GoogleUser::new(
                existing_user._id.to_hex(),
                existing_user.name.to_string(),
                existing_user.email.to_string(),
                existing_user.picture.to_string(),
            ),
            existing_user.role.to_string(),
        );
    }

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(&user).await;
    // if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
    //     user_id = inserted_id.to_hex();
    // }

    user
}

// pub async fn get_users(state: &State<AppState>) -> Vec<UserResponse> {
//     let mut users: Vec<UserResponse> = Vec::new();
//     let collection: Collection<User> = get_collection(state, "user").await;
//     let result = collection.find(doc! {}).await;

//     let cursor = match result {
//         Ok(cursor) => cursor,
//         Err(_) => return vec![],
//     };

//     cursor
//         .try_collect()
//         .await
//         .unwrap_or(vec![])
//         .iter()
//         .for_each(|user| {
//             users.push(UserResponse::copy(user));
//         });

//     users
// }

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<User> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("tasks");
    db.collection::<User>(collection)
}
