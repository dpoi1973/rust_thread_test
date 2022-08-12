use crate::error::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use hello::hello_client::HelloClient;
use hello::HelloRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

pub async fn send_order_to_game(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    println!("order:");
    let mut client = HelloClient::connect("http://127.0.0.1:17076").await?;

    let request = tonic::Request::new(HelloRequest {});
    let response = client.hello_world(request).await?;
    println!("get callback {:?}", response);
    Err(MyError::NotFound("user not found".into()))
    // HttpResponse::Ok().json(response)
    // let response = client.hello_world(request).await?;
    // HttpResponse::Ok().json({})
}
