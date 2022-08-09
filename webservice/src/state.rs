use std::sync::Mutex;
use mongodb::Client;
use redis::Client as redisCli;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub mongodb: Client,
    pub redisdb: redisCli,
}
