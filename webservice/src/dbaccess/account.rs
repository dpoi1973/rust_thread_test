use crate::error::MyError;
use crate::models::account::{Account, CreateAccount};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};

const DATABASE: &str = "apowo";
const TABLE: &str = "accounts";
//添加账号
pub async fn create_account(client: &Client, account: CreateAccount) -> Result<Account, MyError> {
    let conn: Collection<Account> = client.database(DATABASE).collection(TABLE);
    let row = conn
        .find_one(doc! { "extra_id": &account.extra_id }, None)
        .await?;

    Err(MyError::NotFound("account not found".into()))
}

//根据名称查询单个账号
pub async fn get_account_by_username(
    client: &Client,
    username: String,
) -> Result<Account, MyError> {
    let conn: Collection<Account> = client.database(DATABASE).collection(TABLE);

    let row = conn.find_one(doc! { "username": &username }, None).await?;
    if let Some(user) = row {
        Ok(Account {
            _id: user._id,
            username: user.username,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    } else {
        Err(MyError::NotFound("account not found".into()))
    }
}

//根据名称批量查询
pub async fn get_account_list_by_username(
    client: &Client,
    username: String,
) -> Result<Vec<Account>, MyError> {
    let conn: Collection<Account> = client.database(DATABASE).collection(TABLE);

    // let mut rediscoo = redis.get_tokio_connection().await?;
    // let pp = rediscoo.set("key", "value").await?;
    // let ppp: String = rediscoo.get("key").await?;

    let row: Vec<Account> = conn
        .find(doc! { "username": {"$regex": &username }}, None)
        .await?
        .try_collect()
        .await?;
    Ok(row)

    // let result: Vec<Account> = row
    //     .iter()
    //     .map(|user| Account {
    //         _id: user._id,
    //         username: user.username.clone(),
    //         role: user.role,
    //         created_at: user.created_at,
    //         updated_at: user.updated_at,
    //     })
    //     .collect();

    // match result.len() {
    //     0 => Err(MyError::NotFound("account not found".into())),
    //     _ => Ok(result),
    // }
}
