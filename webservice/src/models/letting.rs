use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestAccount {
    pub userId: String,
    pub game: String,
    pub channelNo: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseAccount {
    pub data: Option<ResponseAccountData>,
    pub message: Option<String>,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseAccountData {
    pub age: u32,
    pub auth: u32,
    pub birthDay: Option<String>,
    pub idCard: String,
    pub isGuest: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestOrder {
    pub sign: String,
    pub game: String,
    pub channelNo: String,
    pub gameOrderNo: String,
    pub status: String,
    pub ip: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseOrder {
    pub result: String,
    pub message: String,
}