use crate::dbaccess::account::*;
use crate::error::MyError;
use crate::models::account::{Account, CreateAccount};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn leiting_login_account(
    new_account: web::Json<CreateAccount>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("create account ");
    create_account(&app_state.mongodb, new_account.try_into()?)
        .await
        .map(|result| HttpResponse::Ok().json(result))
}

pub async fn get_account(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, MyError> {
    let username: String = params.into_inner();
    get_account_by_username(&app_state.mongodb, username)
        .await
        .map(|result| HttpResponse::Ok().json(result))
}

pub async fn get_account_list(
    app_state: web::Data<AppState>,
    new_account: web::Json<Account>,
) -> Result<HttpResponse, MyError> {
    let username: String = new_account.username.clone();
    get_account_list_by_username(&app_state.mongodb, username)
        .await
        .map(|result| HttpResponse::Ok().json(result))
}
