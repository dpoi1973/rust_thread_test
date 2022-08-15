use crate::error::MyError;
use crate::models::account::CreateAccount;
use mongodb::bson::doc;
use mongodb::{bson::DateTime, options::UpdateModifications, Client, Collection};

// #[allow(dead_code)]
const _DATABASE: &str = "rust_pkt";
const _TABLE: &str = "accounts";

//数据库创建或更新雷霆账号
pub async fn create_account(client: &Client, account: CreateAccount) -> Result<(), MyError> {
    let conn: Collection<CreateAccount> = client.database(_DATABASE).collection(_TABLE);

    let row = conn
        .find_one(doc! {"userId": &account.userId}, None)
        .await?;
    println!("start mongodb");
    match row {
        Some(user) => {
            let update = doc! {
                "$set": {
                "userId": user.userId.clone(),
                "auth": account.auth,
                "age": account.age,
                "isGuest": account.isGuest,
                "channelNo": account.channelNo.clone(),
                "idCard": account.idCard.clone(),
                "birthDay": account.birthDay.clone(),
                "updatedAt": DateTime::now()
                }
            };
            let update = UpdateModifications::Document(update);
            println!("update account");
            conn.update_one(doc! {"userId": &account.userId.clone()}, update, None)
                .await?;
        }
        None => {
            println!("crete new account");
            conn.insert_one(account.clone(), None).await?;
        }
    }
    Ok(())
}

// //根据名称查询单个账号
// pub async fn get_account_by_username(
//     client: &Client,
//     username: String,
// ) -> Result<Account, MyError> {
//     let conn: Collection<Account> = client.database(DATABASE).collection(TABLE);

//     let row = conn.find_one(doc! { "username": &username }, None).await?;
//     if let Some(user) = row {
//         Ok(Account {
//             _id: user._id,
//             username: user.username,
//             role: user.role,
//             created_at: user.created_at,
//             updated_at: user.updated_at,
//         })
//     } else {
//         Err(MyError::NotFound("account not found".into()))
//     }
// }

// //根据名称批量查询
// pub async fn get_account_list_by_username(
//     client: &Client,
//     username: String,
// ) -> Result<Vec<Account>, MyError> {
//     let conn: Collection<Account> = client.database(DATABASE).collection(TABLE);

//     // let mut rediscoo = redis.get_tokio_connection().await?;
//     // let pp = rediscoo.set("key", "value").await?;
//     // let ppp: String = rediscoo.get("key").await?;

//     let row: Vec<Account> = conn
//         .find(doc! { "username": {"$regex": &username }}, None)
//         .await?
//         .try_collect()
//         .await?;
//     Ok(row)

//     // let result: Vec<Account> = row
//     //     .iter()
//     //     .map(|user| Account {
//     //         _id: user._id,
//     //         username: user.username.clone(),
//     //         role: user.role,
//     //         created_at: user.created_at,
//     //         updated_at: user.updated_at,
//     //     })
//     //     .collect();

//     // match result.len() {
//     //     0 => Err(MyError::NotFound("account not found".into())),
//     //     _ => Ok(result),
//     // }
// }
