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
    //读取.env环境变量
    dotenv().ok();
    let account_port = env::var("ACCOUNT_PORT").expect("has no ACCOUNT_PORT");
    let mongourl = env::var("MONGO_URL").expect("has no MONGO_URL");
    let redisurl = env::var("REDIS_URL").expect("has no REDIS_URL");

    //建立mongo链接
    let mongo_client = Client::with_uri_str(mongourl)
        .await
        .expect("failed to connect mongodb");

    //建立redis链接
    let redis_client = redis::Client::open(redisurl).unwrap();

    //new一个可传入http多线程的结构体
    let shared_data = web::Data::new(AppState {
        health_check_response: "OK ".to_string(),
        visit_count: Mutex::new(0),
        mongodb: mongo_client.clone(),
        redisdb: redis_client.clone(),
    });

    //开启多线程注册路由
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(account_routes)
    };

    //启动账号服务
    println!("http account_server start at {:?}", account_port);
    HttpServer::new(app).bind(account_port)?.run().await
}
