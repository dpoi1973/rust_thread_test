use crate::handlers::account::*;
use crate::handlers::general::*;
use crate::handlers::order::*;
use actix_web::web;

//通用路由
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

//账号路由
pub fn account_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .route("/lt_login", web::post().to(leiting_login_account))
            // .route("/{username}", web::get().to(get_account))
            // .route("/test", web::post().to(get_account_list))
    );
}

//订单路由
pub fn order_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/order")
            .route("/lt_order", web::post().to(get_order_from_leiting))
    );
}
