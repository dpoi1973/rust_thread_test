use crate::error::MyError;
use crate::models::order::{
    CreateLeitingOrder, GetOrderInfo, PaymentStatus, UpdateLeitingOrderStatus,
};
use mongodb::bson::{self, doc};
use mongodb::{bson::DateTime, options::UpdateModifications, Client, Collection};

const _DATABASE: &str = "rust_pkt";
const _TABLE: &str = "orders";

//数据库创建订单
pub async fn create_order(
    client: &Client,
    order: CreateLeitingOrder,
) -> Result<CreateLeitingOrder, MyError> {
    let conn: Collection<CreateLeitingOrder> = client.database(_DATABASE).collection(_TABLE);
    let result = conn.insert_one(order.clone(), None).await?;
    let oo: CreateLeitingOrder = bson::from_bson(result.inserted_id)?;
    Ok(oo)
}

//根据用户查询订单
pub async fn find_order_by_user(client: &Client, user_id: String) -> Result<GetOrderInfo, MyError> {
    let conn: Collection<GetOrderInfo> = client.database(_DATABASE).collection(_TABLE);
    let row = conn.find_one(doc! { "extra_user": &user_id, "payment_status": PaymentStatus::PendingPayment._as_str() }, None).await?;
    if let Some(order) = row {
        Ok(GetOrderInfo {
            _id: order._id,
            extra_type: order.extra_type,
            extra_user: order.extra_user,
            product_id: order.product_id,
            channel_no: order.channel_no,
            payment_status: order.payment_status,
            settlement_status: order.settlement_status,
            order_status: order.order_status,
        })
    } else {
        Err(MyError::NotFound("order not found".into()))
    }
}

//根据订单id查询
pub async fn find_order_by_id(client: &Client, order_id: String) -> Result<GetOrderInfo, MyError> {
    let conn: Collection<GetOrderInfo> = client.database(_DATABASE).collection(_TABLE);

    let row = conn.find_one(doc! { "_id": &order_id }, None).await?;
    if let Some(order) = row {
        Ok(GetOrderInfo {
            _id: order._id,
            extra_type: order.extra_type,
            extra_user: order.extra_user,
            product_id: order.product_id,
            channel_no: order.channel_no,
            payment_status: order.payment_status,
            settlement_status: order.settlement_status,
            order_status: order.order_status,
        })
    } else {
        Err(MyError::NotFound("order not found".into()))
    }
}

//根据id更新订单状态
pub async fn update_order_status_by_id(
    client: &Client,
    order_id: String,
    settlement_status: String,
    order_status: String,
) -> Result<(), MyError> {
    let conn: Collection<UpdateLeitingOrderStatus> = client.database(_DATABASE).collection(_TABLE);

    let update = doc! {
            "$set": {
            "settlement_status": &settlement_status,
            "order_status": &order_status,
            "updated_at": DateTime::now(),
        }
    };
    let update = UpdateModifications::Document(update);
    conn.update_one(doc! { "_id": &order_id }, update, None)
        .await?;
    Ok(())
}
