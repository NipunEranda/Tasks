use std::sync::Arc;

mod models;
mod services;
mod routes;
mod utils;

use dotenv::dotenv;
use mongodb::Client;
use rocket::tokio::sync::Mutex;
use rocket::{launch, routes};

pub struct AppState {
    mongo_client: Arc<Mutex<Client>>,
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let mongo_client = Client::with_options(
        mongodb::options::ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap(),
    )
    .unwrap();

    rocket::build()
        .manage(AppState {
            mongo_client: Arc::new(Mutex::new(mongo_client)),
        })
        .mount(
            "/api/v1/",
            routes![
                routes::user::login,
                routes::workspace::index
            ],
        )
}
