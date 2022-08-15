use mongodb::bson::doc;
use mongodb::{Client, Collection};

use crate::error::MyError;
use crate::models::whiteserver::WhiteServer;

const _DATABASE: &str = "rust_pkt";
const _TABLE: &str = "whiteservers";

//根据名称查询白名单服务
pub async fn get_white_server_by_name(
    client: &Client,
    name: String,
) -> Result<WhiteServer, MyError> {
    let conn: Collection<WhiteServer> = client.database(_DATABASE).collection(_TABLE);

    let row = conn.find_one(doc! { "name": &name }, None).await?;
    if let Some(server) = row {
        Ok(WhiteServer {
            name: server.name,
            servers: server.servers
        })
    } else {
        Err(MyError::NotFound("account not found".into()))
    }
}
