use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

//雷霆订单回调
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LeitingOrder {
    pub currency: String,
    pub gameCoin: String,
    pub gameOrderNo: String,
    pub message: String,
    pub productId: String,
    pub sign: String,
    pub status: String,
    pub thirdAmount: String,
    pub thirdNo: String,
    pub channelNo: String,
    pub userId: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateLeitingOrderReq {
    pub user_id: String,
    pub product_id: String,
    pub channel_no: String,
}

//创建雷霆订单
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateLeitingOrder {
    pub extra_user: String,
    pub payment_status: String,
    pub settlement_status: String,
    pub order_status: String,
    pub product_id: String,
    pub extra_type: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

//获取订单信息
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetOrderInfo {
    pub _id: Option<ObjectId>,
    pub extra_user: String,
    pub payment_status: String,
    pub settlement_status: String,
    pub order_status: String,
    pub product_id: String,
    pub extra_type: String,
    pub channel_no: String,
}

//更新雷霆订单状态
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateLeitingOrderStatus {
    pub payment_status: String,
    pub settlement_status: String,
    pub order_status: String,
    // pub product_id: String,
    // pub extra_type: String,
    pub updated_at: DateTime,
}

#[allow(dead_code)]
pub enum PaymentStatus {
    PendingPayment,
    PaymentSuccess,
    PaymentFailed,
}
impl PaymentStatus {
    pub fn _as_str(&self) -> &'static str {
        match self {
            PaymentStatus::PendingPayment => "pending", //待支付
            PaymentStatus::PaymentSuccess => "success", //支付成功
            PaymentStatus::PaymentFailed => "failed",
        }
    }
}
#[allow(dead_code)]
pub enum SettlementStatus {
    PendingSettle,
    Failed,
    Complete,
}
impl SettlementStatus {
    pub fn _as_str(&self) -> &'static str {
        match self {
            SettlementStatus::PendingSettle => "pending", //待结算
            SettlementStatus::Failed => "failed",         //回款失败
            SettlementStatus::Complete => "complete",     //回款成功
        }
    }
}

#[allow(dead_code)]
pub enum OrderStatus {
    PendingProcess,
    Refounded,
    Complete,
}
impl OrderStatus {
    pub fn _as_str(&self) -> &'static str {
        match self {
            OrderStatus::PendingProcess => "pending_process", //待支付
            OrderStatus::Refounded => "refunded",             //已退款
            OrderStatus::Complete => "complete",              //已完成
        }
    }
}
