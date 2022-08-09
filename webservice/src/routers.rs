use crate::handlers::account::*;
use crate::handlers::general::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn account_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .route("/", web::post().to(leiting_login_account))
            .route("/{username}", web::get().to(get_account))
            .route("/test", web::post().to(get_account_list))
    );
}
