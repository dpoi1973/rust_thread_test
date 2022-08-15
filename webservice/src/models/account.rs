use crate::error::MyError;
use actix_web::web;
use mongodb::bson::{doc,oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

//数据库accounts字段
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub _id: Option<ObjectId>,
    pub userId: String,
    pub auth: Option<u32>,
    pub age: Option<u32>,
    pub isGuest: Option<u32>,
    pub channelNo: Option<String>,
    pub idCard: Option<String>,
    pub birthDay: Option<String>,
    pub userName: Option<String>,
    pub nickName: Option<String>,
    pub createdAt: Option<DateTime>,
    pub updatedAt: Option<DateTime>,
}

//验证雷霆账号request
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LeiTingAccount {
    pub userId: String,
    pub token: String,
    pub channelNo: String,
    pub userName: Option<String>,
    pub nickName: Option<String>,
}

impl TryFrom<web::Json<LeiTingAccount>> for LeiTingAccount {
    type Error = MyError;

    fn try_from(account: web::Json<LeiTingAccount>) -> Result<Self, Self::Error> {
        println!("das");
        Ok(LeiTingAccount {
            userId: account.userId.clone(),
            token: account.token.clone(),
            channelNo: account.channelNo.clone(),
            userName: account.userName.clone(),
            nickName: account.nickName.clone(),
        })
    }
}

//验证雷霆账号response
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LeiTingAccountResp {
    pub userId: String,
    pub token: String,
}

impl From<web::Json<LeiTingAccountResp>> for LeiTingAccountResp {
    fn from(account: web::Json<LeiTingAccountResp>) -> Self {
        LeiTingAccountResp {
            userId: account.userId.clone(),
            token: account.token.clone(),
        }
    }
}

//创建雷霆账号
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateAccount {
    pub userId: String,
    pub auth: u32,
    pub age: u32,
    pub isGuest: u32,
    pub channelNo: String,
    pub idCard: String,
    pub birthDay: Option<String>,
    pub createdAt: DateTime,
}
