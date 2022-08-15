use serde::{Deserialize, Serialize};

//数据库whiteservers字段
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WhiteServer {
    pub name: String,
    pub servers: Vec<String>,
}
