use crate::error::MyError;
use actix_web::web;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub _id: Option<ObjectId>,
    pub username: String,
    pub role: Option<u8>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateAccount {
    pub username: String,
    pub id_card: Option<String>,
    pub birth_day: Option<String>,
    pub phone: Option<String>,
    pub role: u8,
    pub extra_id: Option<String>,
    pub extra_type: Option<String>,
    pub auth: Option<u8>,
    pub age: Option<u32>,
}

impl TryFrom<web::Json<CreateAccount>> for CreateAccount {
    type Error = MyError;
    fn try_from(account: web::Json<CreateAccount>) -> Result<Self, Self::Error> {
        Ok(CreateAccount {
            username: account.username.clone(),
            role: account.role,
            id_card: account.id_card.clone(),
            birth_day: account.birth_day.clone(),
            phone: account.phone.clone(),
            extra_id: account.extra_id.clone(),
            extra_type: account.extra_type.clone(),
            auth: account.auth,
            age: account.age,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateAccount {
    pub username: String,
    pub id_card: Option<String>,
    pub birth_day: Option<String>,
    pub phone: Option<String>,
    pub role: Option<u8>,
    pub extra_id: Option<String>,
    pub extra_type: Option<String>,
    pub auth: Option<u8>,
    pub age: Option<u32>,
}

impl From<web::Json<UpdateAccount>> for UpdateAccount {
    fn from(account: web::Json<UpdateAccount>) -> Self {
        UpdateAccount {
            username: account.username.clone(),
            role: account.role,
            id_card: account.id_card.clone(),
            birth_day: account.birth_day.clone(),
            phone: account.phone.clone(),
            extra_id: account.extra_id.clone(),
            extra_type: account.extra_type.clone(),
            auth: account.auth,
            age: account.age,
        }
    }
}