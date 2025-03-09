use crate::modules::authentication::model::*;
use crate::modules::authentication::service::*;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
#[derive(OpenApi)]
#[openapi(
    paths(register_user, get_all_users, login),
    components(schemas(NewUser, LoginRequest))
)]
pub struct ApiDoc;

pub fn swagger_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
