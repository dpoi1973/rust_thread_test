use actix_web::{error, http::StatusCode, HttpResponse, Result};
use awc::error::{JsonPayloadError, SendRequestError};
use mongodb::error::Error as MongoErr;
use redis::RedisError;
use serde::Serialize;
use std::fmt;

//自定义error类型，手动实现impl from 即可在async里使用？表示错误捕获

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    BadRequest(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal Server error".into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            MyError::BadRequest(msg) => {
                println!("Bad request: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadRequest(_msg) => StatusCode::BAD_REQUEST,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<MongoErr> for MyError {
    fn from(err: MongoErr) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl From<RedisError> for MyError {
    fn from(err: RedisError) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl From<tonic::Status> for MyError {
    fn from(err: tonic::Status) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl From<tonic::transport::Error> for MyError {
    fn from(err: tonic::transport::Error) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl From<SendRequestError> for MyError {
    fn from(err: SendRequestError) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<JsonPayloadError> for MyError {
    fn from(err: JsonPayloadError) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<error::PayloadError> for MyError {
    fn from(err: error::PayloadError) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<mongodb::bson::ser::Error> for MyError {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl From<mongodb::bson::de::Error> for MyError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> Self {
        MyError::BadRequest(err.to_string())
    }
}
