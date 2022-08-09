use serde_json::json;

use crate::{
    error::MyError,
    models::letting::{RequestAccount, ResponseAccount, ResponseAccountData},
};

pub async fn get_account_from_leiting(
    userId: String,
    channelNo: String,
    token: String,
) -> Result<ResponseAccountData, MyError> {
    let awc_client = awc::Client::default();

    let value = RequestAccount {
        userId,
        game: String::from(""),
        channelNo,
        token,
    };

    let res = awc_client
        .post("url")
        .insert_header(("Content-Type", "application/json"))
        .send_json(&value)
        // .send()
        .await
        .unwrap()
        .json::<ResponseAccount>()
        .await
        .unwrap();
    if res.status == "success" {
        if let Some(r) = res.data {
            Ok(r)
        } else {
            Err(MyError::NotFound("user not found".into()))
        }
    } else {
        Err(MyError::NotFound("user not found".into()))
    }
}
