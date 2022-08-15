use serde::{Deserialize, Serialize};

//雷霆账号返回
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseAccount {
    pub data: Option<ResponseAccountData>,
    pub message: Option<String>,
    pub status: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseAccountData {
    pub age: u32,
    pub auth: u32,
    pub birthDay: Option<String>,
    pub idCard: String,
    pub isGuest: u32,
}

//雷霆订单异常上报请求
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestOrder {
    pub sign: String,
    pub game: String,
    pub channelNo: String,
    pub gameOrderNo: String,
    pub status: String,
    pub ip: Option<String>
}

//雷霆订单异常上报返回
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseOrder {
    pub result: String,
    pub message: String,
}