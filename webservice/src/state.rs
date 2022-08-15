use mongodb::Client;
use redis::Client as redisCli;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub mongodb: Client,
    pub redisdb: redisCli,
}
