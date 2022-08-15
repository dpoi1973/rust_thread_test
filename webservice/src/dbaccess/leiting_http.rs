use actix_web::http::StatusCode;

use crate::{
    error::MyError,
    models::letting::{ResponseAccount, ResponseAccountData},
};

//雷霆账号验证请求
#[allow(non_snake_case)]
pub async fn get_account_from_leiting(
    userId: String,
    channelNo: String,
    token: String,
) -> Result<ResponseAccountData, MyError> {
    let awc_client = awc::Client::default();
    let params = [
        ("userId", userId.to_string()),
        ("game", "pktzmxqy".to_string()),
        ("channelNo", channelNo.to_string()),
        ("token", token.to_string()),
    ];
    // https://loginpktzmxqy.leiting.com/login/verify/verifyToken.do

    let mut res = awc_client
        .post("https://loginpktzmxqy.leiting.com/login/verify/verifyToken.do")
        .send_form(&params)
        .await?;

    match res.status() {
        StatusCode::OK => {
            let k = res.body().limit(20_000_000).await?;
            let body_str = String::from_utf8_lossy(&k);
            println!("get from leiting{:?}",body_str);
            let result: ResponseAccount = serde_json::from_str(&body_str)?;
            if result.status == "fail" {
                Err(MyError::ActixError(
                    "can not get user info from leiting".into(),
                ))
            } else if let Some(user) = result.data {
                Ok(ResponseAccountData {
                    age: user.age,
                    auth: user.auth,
                    birthDay: user.birthDay,
                    idCard: user.idCard,
                    isGuest: user.isGuest,
                })
            } else {
                Err(MyError::ActixError(
                    "can not get user info from leiting".into(),
                ))
            }
        }
        _ => Err(MyError::ActixError(
            "can not get user info from leiting".into(),
        )),
    }
}
