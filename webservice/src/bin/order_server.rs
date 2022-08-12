use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::Client;
use routers::*;
use state::AppState;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;

#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../routers.rs"]
mod routers;

#[path = "../state.rs"]
mod state;

#[path = "../models/mod.rs"]
mod models;

#[path = "../error.rs"]
mod error;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let account_port = env::var("ACCOUNT_PORT").expect("has no ACCOUNT_PORT");
    let mongourl = env::var("MONGO_URL").expect("has no MONGO_URL");
    let redisurl = env::var("REDIS_URL").expect("has no REDIS_URL");

    let mongo_client = Client::with_uri_str(mongourl)
        .await
        .expect("failed to connect mongodb");

    let redis_client = redis::Client::open(redisurl).unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "OK ".to_string(),
        visit_count: Mutex::new(0),
        mongodb: mongo_client.clone(),
        redisdb: redis_client.clone(),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(order_routes)
    };
    println!("account_server start at {:?}", account_port);
    HttpServer::new(app).bind(account_port)?.run().await
}
