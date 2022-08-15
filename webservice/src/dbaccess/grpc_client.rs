use chrono::prelude::*;
use game_clinet::game_grpc_service_client::GameGrpcServiceClient;
use game_clinet::{DeliverOrderRequest, OrderProduct};

use crate::error::MyError;

pub mod game_clinet {
    tonic::include_proto!("grpc_game_proto");
}

pub async fn send_order_to_game(
    order_id: String,
    platform_id: String,
    price: i32,
    product_id: String,
    character_id: i64,
) -> Result<bool, MyError> {
    let mut client = GameGrpcServiceClient::connect("http://127.0.0.1:17076").await?;
    let request = tonic::Request::new(DeliverOrderRequest {
        order_id: order_id,
        create_time: Local::now().timestamp_millis(),
        platform_id: platform_id,
        product: Some(OrderProduct {
            price: price,
            id: product_id,
        }),
        character_id: character_id,
    });
    let response = client.deliver_order(request).await?;
    println!("get callback {:?}", response);

    if response.into_inner().success {
        Ok(true)
    } else {
        Ok(false)
    }
}
