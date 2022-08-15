use crate::dbaccess::{grpc_client::*, order::*, whiteserver::*};
use crate::error::MyError;
use crate::models::order::{
    CreateLeitingOrder, CreateLeitingOrderReq, LeitingOrder, OrderStatus, PaymentStatus,
    SettlementStatus,
};
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use mongodb::bson::DateTime;

const _SECRET: &str = "";

//创建订单
pub async fn create_leiting_order(
    new_order: web::Json<CreateLeitingOrderReq>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let result = find_order_by_user(&app_state.mongodb, new_order.user_id.to_string()).await;
    match result {
        Ok(order) => return Ok(HttpResponse::Ok().json(order)),
        Err(_err) => {
            let order = CreateLeitingOrder {
                extra_user: new_order.user_id.to_string(),
                product_id: new_order.product_id.to_string(),
                extra_type: new_order.channel_no.to_string(),
                payment_status: PaymentStatus::PendingPayment._as_str().to_string(),
                settlement_status: SettlementStatus::PendingSettle._as_str().to_string(),
                order_status: OrderStatus::PendingProcess._as_str().to_string(),
                created_at: DateTime::now(),
                updated_at: DateTime::now(),
            };
            let result = create_order(&app_state.mongodb, order).await;
            match result {
                Ok(order) => Ok(HttpResponse::Ok().json(order)),
                _ => Err(MyError::DBError("create order failed".into())),
            }
        }
    }
}

//雷霆订单回调
pub async fn get_order_from_leiting(
    req: HttpRequest,
    new_order: web::Json<LeitingOrder>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("new order in: {:?}", new_order);

    //验证签名
    let sign_str = format!(
        "{}{}{}{}{}{}{}{}{}",
        new_order.status,
        new_order.gameOrderNo,
        new_order.thirdNo,
        new_order.channelNo,
        new_order.currency,
        new_order.thirdAmount,
        new_order.productId,
        new_order.userId,
        _SECRET,
    );
    let sign_check = md5::compute(sign_str);

    if format!("{:x}", sign_check) != new_order.sign {
        return Err(MyError::BadRequest("sign error".into()));
    }

    //验证白名单
    let mut client_ip: String = String::from("");
    if let Some(ip) = req.connection_info().realip_remote_addr() {
        client_ip = ip.to_string();
    }
    if let Some(ip) = req.connection_info().peer_addr() {
        client_ip = ip.to_string();
    }
    let is_server = get_white_server_by_name(&app_state.mongodb, "leiting".to_string()).await?;

    if !is_server.servers.contains(&client_ip) {
        return Err(MyError::BadRequest("bad request!".into()));
    }

    //失败订单回调
    if new_order.status != "success" {
        update_order_status_by_id(
            &app_state.mongodb,
            new_order.gameOrderNo.clone(),
            SettlementStatus::Failed._as_str().to_string(),
            OrderStatus::Refounded._as_str().to_string(),
        )
        .await?;
        return Ok(HttpResponse::Ok().json({}));
    }

    let order = find_order_by_id(&app_state.mongodb, new_order.gameOrderNo.to_string()).await?;

    //验证订单状态
    //订单已完成直接返回
    if order.order_status == OrderStatus::Complete._as_str() {
        return Ok(HttpResponse::Ok().json(order));
    }

    //结算状态不是pending也返回
    if order.settlement_status != SettlementStatus::PendingSettle._as_str() {
        return Err(MyError::BadRequest("err order".into()));
    }

    //验证用户
    if order.extra_user != new_order.userId {
        return Err(MyError::BadRequest("err user".into()));
    }

    //验证商品
    if order.product_id != new_order.productId {
        return Err(MyError::BadRequest("err product".into()));
    }

    //验证渠道
    if order.extra_type != new_order.channelNo {
        return Err(MyError::BadRequest("err channel".into()));
    }

    update_order_status_by_id(
        &app_state.mongodb,
        new_order.gameOrderNo.clone(),
        SettlementStatus::Complete._as_str().to_string(),
        OrderStatus::PendingProcess._as_str().to_string(),
    )
    .await?;

    let send_res = send_order_to_game(
        new_order.thirdNo.clone(),
        new_order.gameOrderNo.clone(),
        -1,
        new_order.productId.clone(),
        -1,
    )
    .await;
    match send_res {
        Ok(result) => {
            if result == true {
                update_order_status_by_id(
                    &app_state.mongodb,
                    new_order.gameOrderNo.clone(),
                    SettlementStatus::Failed._as_str().to_string(),
                    OrderStatus::Refounded._as_str().to_string(),
                )
                .await?;
                Ok(HttpResponse::Ok().json(order))
            } else {
                Ok(HttpResponse::Ok().json(order))
            }
        }
        Err(_e) => Ok(HttpResponse::Ok().json(order)),
    }
}
