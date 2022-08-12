use crate::dbaccess::account_redis::*;
use dotenv::dotenv;
use redis::Client;
use std::env;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use AccountGrpc::account_server::{Account, AccountServer};
use AccountGrpc::{ConnectReply, ConnectRequest, LoginReply, LoginRequest};

#[path = "../dbaccess/mod.rs"]
mod dbaccess;

#[path = "../error.rs"]
mod error;

#[path = "../models/mod.rs"]
mod models;

pub mod AccountGrpc {
    tonic::include_proto!("grpc_proto");
}

// #[derive(Default)]
pub struct MyServer {
    redis: Client,
}

#[tonic::async_trait]
impl Account for MyServer {
    async fn connect_server(
        &self,
        req: Request<ConnectRequest>,
    ) -> Result<Response<ConnectReply>, Status> {
        let request = req.into_inner();
        let result = save_game_server(
            self.redis.clone(),
            request.server_name,
            request.server_address,
        )
        .await;
        match result {
            Ok(k) => {
                let response = ConnectReply {
                    success: true,
                    message: "connect ok".to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                let response = ConnectReply {
                    success: false,
                    message: "Cannot save server".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn login_check(
        &self,
        req: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        let result = check_token(self.redis.clone(), req.into_inner().token).await;
        match result {
            Ok(user) => {
                let response = LoginReply {
                    success: true,
                    user_id: user.userId.clone(),
                    message: "got user success".to_string(),
                };
                Ok(Response::new(response))
            }
            _ => {
                let response = LoginReply {
                    success: false,
                    user_id: "none".to_string(),
                    message: "use not found".to_string(),
                };
                Ok(Response::new(response))
            }
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let redisurl = env::var("REDIS_URL").expect("has no REDIS_URL");
    let grpc_port = env::var("GRPC_PORT").expect("has no GRPC_PORT");

    //建立redis链接
    let redis_client = redis::Client::open(redisurl).unwrap();

    let addr = grpc_port.parse()?;
    let account_server = MyServer {
        redis: redis_client.clone(),
    };
    println!("grpc_server start at {:?}", grpc_port);

    Server::builder()
        .add_service(AccountServer::new(account_server))
        .serve(addr)
        .await?;
    Ok({})

    // let mut server=grpc::ServerBuilder::new_plain();

    // server.http.set_addr("0.0.0.0:30303");
    // server.http.set_cpu_pool_threads(4);
    // server.add_service(HelloServiceServer::new_service_def(Hello));
    // let _server:Server = server.build().expect("server");

    // loop{
    //     thread::park();
    // }
}
