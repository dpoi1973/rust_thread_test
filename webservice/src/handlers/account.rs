use crate::dbaccess::{account::*, redis::*, leiting_http::*};
use crate::error::MyError;
use crate::models::account::{CreateAccount, LeiTingAccount, LeiTingAccountResp};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use mongodb::bson::DateTime;

//雷霆账号登陆后验证
pub async fn leiting_login_account(
    new_account: web::Json<LeiTingAccount>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("leiting login :");
    //redis 验证是否token已存在
    let is_token = check_token(
        app_state.redisdb.clone(),
        new_account.token.clone(),
    )
    .await;

    match is_token {
        Ok(user) => {
            if user.userId == new_account.userId {
                HttpResponse::Ok().json(user);
            } else {
                return Err(MyError::BadRequest("error user".into()));
            }
        }
        _ => {}
    };

    //雷霆验证查询
    let result = get_account_from_leiting(
        new_account.userId.clone(),
        new_account.channelNo.clone(),
        new_account.token.clone(),
    )
    .await;
    match result {
        Ok(user) => {
            //塞进数据库
            let acc = CreateAccount {
                userId: new_account.userId.clone(),
                auth: user.auth,
                age: user.age,
                isGuest: user.isGuest.clone(),
                channelNo: new_account.channelNo.clone(),
                idCard: user.idCard,
                birthDay: user.birthDay,
                createdAt: DateTime::now(),
            };
            create_account(&app_state.mongodb, acc).await?;
            add_token(
                app_state.redisdb.clone(),
                new_account.token.clone(),
                new_account.userId.clone(),
            )
            .await?;
            Ok(HttpResponse::Ok().json(LeiTingAccountResp {
                userId: new_account.userId.clone(),
                token: new_account.token.clone(),
            }))
        }
        _ => Err(MyError::NotFound("user not found".into())),
    }
}

// pub async fn get_account(
//     app_state: web::Data<AppState>,
//     params: web::Path<String>,
// ) -> Result<HttpResponse, MyError> {
//     let username: String = params.into_inner();
//     get_account_by_username(&app_state.mongodb, username)
//         .await
//         .map(|result| HttpResponse::Ok().json(result))
// }

// pub async fn get_account_list(
//     app_state: web::Data<AppState>,
//     new_account: web::Json<Account>,
// ) -> Result<HttpResponse, MyError> {
//     let username: String = new_account.username.clone();
//     get_account_list_by_username(&app_state.mongodb, username)
//         .await
//         .map(|result| HttpResponse::Ok().json(result))
// }
