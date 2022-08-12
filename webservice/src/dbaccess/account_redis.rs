use redis::AsyncCommands;
use redis::Client;

use crate::error::MyError;
use crate::models::account::LeiTingAccountResp;

const REDIS_H_SIGNIN_SESSION: &str = "_*_Session_TK_";
const REDIS_H_GAME_SERVER: &str = "_Game_Servers_";

//检查token是否存在
pub async fn check_token(redis: Client, token: String) -> Result<LeiTingAccountResp, MyError> {
    // let redis_token = REDIS_H_SIGNIN_SESSION.to_string() + &token;
    let redis_conn = &mut redis.get_tokio_connection().await?;

    let user_cookie: Result<String, redis::RedisError> = redis_conn
        .hget(format!("{}:{}", REDIS_H_SIGNIN_SESSION, token), "userId")
        .await;
    match user_cookie {
        Ok(user_id) => Ok(LeiTingAccountResp {
            userId: user_id,
            token,
        }),
        Err(err) => Err(MyError::NotFound("token not exists".into())),
    }
}

//插入token
pub async fn add_token(redis: Client, token: String, userId: String) -> Result<(), MyError> {
    let redis_conn = &mut redis.get_tokio_connection().await?;
    let _: () = redis_conn
        .hset_multiple(
            format!("{}:{}", REDIS_H_SIGNIN_SESSION, token),
            &[("userId", userId)],
        )
        .await?;
    redis_conn
        .expire(format!("{}:{}", REDIS_H_SIGNIN_SESSION, token), 3600)
        .await?;
    Ok(())
}

//维护游戏链接
pub async fn save_game_server(
    redis: Client,
    server_name: String,
    server_address: String,
) -> Result<(), MyError> {
    let redis_conn = &mut redis.get_tokio_connection().await?;
    let _: () = redis_conn
        .set(
            format!("{}:{}", REDIS_H_GAME_SERVER, server_name),
            server_address,
        )
        .await?;
    Ok(())
}
