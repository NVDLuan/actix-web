use actix_web::web;
use crate::modules::authentication::endpoint::init_auth_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    init_auth_routes(cfg);
}
