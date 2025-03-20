use std::env;
use std::sync::Arc;

mod models;
mod services;
mod routes;
mod utils;

use dotenv::dotenv;
use mongodb::Client;
use rocket::tokio::sync::Mutex;
use rocket::{launch, route, routes, Config};

pub struct AppState {
    mongo_client: Arc<Mutex<Client>>,
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let port = env::var("PORT").ok();

    let config = Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", (if !port.is_none() {port.unwrap()} else {String::from("8000")}).parse::<i16>().ok().unwrap()));

    let mongo_client = Client::with_options(
        mongodb::options::ClientOptions::parse(env::var("MONGO_URL").ok().unwrap())
            .await
            .unwrap(),
    )
    .unwrap();

    rocket::custom(config)
        .manage(AppState {
            mongo_client: Arc::new(Mutex::new(mongo_client)),
        })
        .mount(
            "/api/v1/",
            routes![
                routes::user::get_user,
                routes::user::login,
                routes::user::logout_user,
                routes::workspace::test,
                routes::workspace::get_workspaces,
                routes::workspace::create_workspace,
                routes::workspace::update_workspace,
                routes::workspace::delete_workspace
            ],
        )
}
