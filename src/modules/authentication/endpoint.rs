use actix_web::web;
use crate::modules::authentication::service;

pub fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth")
        .route("/register", web::post().to(service::register_user))
        .route("/users", web::get().to(service::get_all_users))
        .route("/login", web::post().to(service::login))
    );
}
